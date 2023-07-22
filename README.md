# Optimal ETH Trade Profit Calculator

This repository contains a Rust program that simulates trade scenarios between two exchanges and calculates the maximum possible profit and the corresponding amount of Ethereum (ETH) needed to achieve that profit.

## Background

Cryptocurrency exchanges don't always have the same prices for the same assets. This difference is due to the supply and demand of each asset in each exchange. These discrepancies can create opportunities for what is known as arbitrage trading, which is the practice of buying an asset at a lower price in one market and selling it at a higher price in another.

This code simulates a simple scenario where a trader has an initial amount of ETH and wants to see the potential profit from performing arbitrage trading between two exchanges.

## Implementation

The program calculates the profit that can be obtained by selling an increasing amount of ETH in the first exchange for USDC (a stablecoin pegged to the USD), and then using the USDC to buy back ETH in the second exchange. The maximum profit is achieved when the profit stops increasing.

The exchanges have a mechanism called "Automated Market Makers" (AMMs), which adjust the price of the assets according to their reserves. The code simulates the behavior of AMMs and calculates the new reserves after each transaction.

The `reserve1_eth`, `reserve1_usdc`, `reserve2_eth`, and `reserve2_usdc` variables represent the initial reserves of ETH and USDC in the first and second exchanges, respectively.

The `amount_eth_start` and `amount_eth_increment` variables control the increment of the amount of ETH to be sold in each iteration. The loop will stop once the profit stops increasing, indicating that the optimal amount of ETH to sell has been found.

Finally, the maximum profit and the corresponding amount of ETH to sell are printed out.

## How to Run

To run the program, you will need to have Rust installed. After you have installed Rust, you can run the program by navigating to the project directory and using the command:

```sh
cargo run
```

This will compile and run the program, outputting the maximum profit and corresponding amount of ETH to the console.
