# Crypto Arbitrage Analyzer

This repository houses a Rust-based application purpose-built to scrutinize and juxtapose cryptocurrency pairs on Uniswap and Sushiswap. It is specifically developed to compute the reserves and prices of ETH/USDC pairs across both exchanges and pinpoint prospective arbitrage possibilities.

## Features
- Procures real-time reserve information for ETH/USDC pairs hailing from Uniswap and Sushiswap.
- Determines the prevailing value of WETH in terms of USDC on both exchanges.
- Discovers potential arbitrage prospects by contrasting prices.
- Leverages the robust async and concurrency support in Rust for efficient data maneuvering.
- Guarantees precision and security with Rust's stringent type system and ownership construct.

## Usage
To leverage this tool, clone the repository and kickstart the application using `cargo run` command in Rust. Ensure Rust is set up on your system.

## Dependencies
- [`ethers-rs`](https://github.com/gakonst/ethers-rs): A comprehensive Ethereum and Celo library and wallet crafted in Rust.
- Miscellaneous standard Rust libraries and ancillary async support.

## Contributions
Contributions are appreciated! Feel free to submit a pull request or initiate an issue for any enhancements or proposals.

## License
This project falls under the [MIT License](LICENSE).

## Disclaimer
This tool is for educational objectives only. Users are urged to exercise due diligence and perform comprehensive research while trading or investigating cryptocurrency markets.