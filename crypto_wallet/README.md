# Ethereum Wallet in Rust

This project demonstrates how to create, save, and load an Ethereum wallet using Rust. It also includes functionality to connect to an Ethereum blockchain via WebSocket and retrieve the wallet's balance and the current block number.

## Features

- Generate a new Ethereum wallet with a public and private key.
- Save the wallet to a JSON file.
- Load a wallet from a JSON file.
- Connect to an Ethereum blockchain using WebSocket.
- Retrieve the wallet's balance in Ethereum.
- Get the current block number of the Ethereum blockchain.

## Prerequisites

- Rust programming language setup on your machine.
- An Ethereum node is accessible via WebSocket URL. This can be obtained from services like Alchemy or Infura.

## Setup

1. Clone the repository to your local machine.
2. Ensure you have Rust and Cargo installed.
3. Create a `.env` file in the root directory of the project and add your WebSocket URL as follows:
    
    ```
    ALCHEMY_SEPOLIA_WS=your_websocket_url_here
    
    ```
    
4. Run `cargo build` to compile the project.

## Usage

To run the project, use the following command:

```
cargo run

```

This will:

- Generate a new Ethereum wallet (commented out in the provided code).
- Load an existing wallet from `./crypto_wallet.json`.
- Connect to an Ethereum blockchain using the WebSocket URL provided in your `.env` file.
- Print the current block number and the wallet's balance to the console.

## Contributing

Contributions to this project are welcome. Please ensure to follow the code of conduct and submit pull requests for any enhancements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.