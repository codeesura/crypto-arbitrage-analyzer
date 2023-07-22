fn main() {
    // Reserves of ETH and USDC in the first exchange
    let (reserve1_eth, reserve1_usdc) = (3586.0, 6454800.0);
    // Reserves of ETH and USDC in the second exchange
    let (reserve2_eth, reserve2_usdc) = (1123.0, 1998940.0);

    // Initial amount of ETH
    let mut amount_eth_start = 0.0;
    // Amount of ETH to be added in each iteration
    let amount_eth_increment = 0.00001;

    // Maximum profit
    let mut max_profit = 0.0;

    // Loop 500,000 times
    for _ in 0..500000 {
        // Increase the amount of ETH in each iteration
        amount_eth_start += amount_eth_increment;

        // Calculate the new ETH reserves
        let new_reserve1_eth = reserve1_eth - amount_eth_start;
        // Calculate the new USDC reserves and the amount of USDC received
        let new_reserve1_usdc = (reserve1_eth * reserve1_usdc) / (reserve1_eth + amount_eth_start);
        let received_usdc = reserve1_usdc - new_reserve1_usdc;

        // Calculate the new USDC reserves
        let new_reserve2_usdc = reserve2_usdc + received_usdc;
        // Calculate the new ETH reserves and the amount of ETH received
        let new_reserve2_eth = (reserve2_eth * reserve2_usdc) / (reserve2_usdc + received_usdc);
        let received_eth = reserve2_eth - new_reserve2_eth;

        // Calculate the profit
        let profit = received_eth - amount_eth_start;

        // If the profit is greater than the current maximum profit, update the maximum profit
        if profit > max_profit {
            max_profit = profit;
        } else {
            // If the profit stops increasing, stop the loop
            break;
        }
    }

    // Print the maximum profit and the corresponding amount of ETH
    println!("Maximum profit: {}, Corresponding amount of ETH: {}", max_profit, amount_eth_start);
}
