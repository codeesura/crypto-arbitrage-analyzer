use ethers::{
    contract::abigen,
    prelude::*,
    providers::{Http, Provider},
};
use std::{error::Error, sync::Arc};

// Define the contract binding
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

const USDC_DECIMAL: u128 = 1_000_000; // 6 decimals for USDC
const ETH_DECIMAL: u128 = 1_000_000_000_000_000_000; // 18 decimals for ETH

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let http_provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let shared_provider = Arc::new(http_provider);

    // Function to create a Uniswap pair
    let create_pair = |address: &str| -> Result<Arc<IUniswapV2Pair<Provider<Http>>>, Box<dyn Error>> {
        let pair_address = address.parse::<Address>()
            .map_err(|_| "Invalid address for pair")?;
        Ok(Arc::new(IUniswapV2Pair::new(pair_address, Arc::clone(&shared_provider))))
    };

    let uniswap_pair = create_pair("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc")?;
    let sushiswap_pair = create_pair("0x397ff1542f962076d0bfe58ea045ffa2d347aca0")?;    

    let print_reserves = |pair: Arc<IUniswapV2Pair<Provider<Http>>>, name: String| async move {
        let (reserve0, reserve1, _) = pair.get_reserves().call().await?;
        println!(
            "Reserves in {} (Token1, Token2): {}, {}",
            name, reserve0, reserve1
        );
        Ok::<_, Box<dyn Error>>((reserve0, reserve1))
    };

    let (reserves_usdc_uniswap, reserves_weth_uniswap) = print_reserves(uniswap_pair, "Uniswap".to_string()).await?;
    let (reserves_usdc_sushiswap, reserves_weth_sushiswap) = print_reserves(sushiswap_pair, "Sushiswap".to_string()).await?;

    let calculate_price = |reserves_usdc: u128, reserves_weth: u128| -> f64 {
        (reserves_usdc as f64 / USDC_DECIMAL as f64) / (reserves_weth as f64 / ETH_DECIMAL as f64)
    };

    let uniswap_price = calculate_price(reserves_usdc_uniswap, reserves_weth_uniswap);
    let sushiswap_price = calculate_price(reserves_usdc_sushiswap, reserves_weth_sushiswap);
    println!(
        "Uniswap WETH price: {} USDC \nSushiswap WETH price: {} USDC",
        uniswap_price, sushiswap_price
    );

    let fee_ratio: f64 = 0.997; // Uniswap-Sushiswap fixed %0.3 fee (1-r)
    if uniswap_price < sushiswap_price {
        let exchange_amount_uniswap = reserves_weth_uniswap as f64 * reserves_usdc_sushiswap as f64
            / (reserves_usdc_sushiswap as f64 * fee_ratio + reserves_usdc_uniswap as f64);
        let exchange_amount_sushiswap = reserves_usdc_uniswap as f64 * reserves_weth_sushiswap as f64
            / (reserves_usdc_sushiswap as f64 + reserves_usdc_uniswap as f64 * fee_ratio);
    
        let optimal_delta = (exchange_amount_sushiswap * exchange_amount_uniswap * fee_ratio)
            .sqrt()
            - exchange_amount_sushiswap;
        println!(
            "Optimal Delta (Buy Uniswap, sell Sushiswap): {} WETH",
            optimal_delta / ETH_DECIMAL as f64
        );
    } else {
        let exchange_amount_sushiswap = reserves_weth_sushiswap as f64 * reserves_usdc_uniswap as f64
            / (reserves_usdc_uniswap as f64 * fee_ratio + reserves_usdc_sushiswap as f64);
        let exchange_amount_uniswap = reserves_usdc_sushiswap as f64 * reserves_weth_uniswap as f64
            / (reserves_usdc_uniswap as f64 + reserves_usdc_sushiswap as f64 * fee_ratio);
    
        let optimal_delta = (exchange_amount_uniswap * exchange_amount_sushiswap * fee_ratio)
            .sqrt()
            - exchange_amount_uniswap;
        println!(
            "Optimal Delta (Buy Sushiswap, sell Uniswap): {} WETH",
            optimal_delta / ETH_DECIMAL as f64
        );
    }

    Ok(())
}
