#![allow(unused)]

use anyhow::Result;
use wallet::Wallet;
// use eth_wallet::Wallet;
#[allow(unused_imports)]
use std::dbg;
use std::str::FromStr;
use web3::types::Address;

mod wallet;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_endpoint = std::env::var("ALCHEMY_SEPOLIA_WS")?;
    let mut wallet = Wallet::new(&api_endpoint).await.unwrap();

    // adding new account by taking account credentials from a file
    wallet
        .add_from_wallet_file("genesis_wallet", "./crypto_wallet-prv.json")
        .unwrap();

    let balance = wallet.get_balance_in_eth("genesis_wallet").await.unwrap();

    dbg!(balance);

    Ok(())
}
