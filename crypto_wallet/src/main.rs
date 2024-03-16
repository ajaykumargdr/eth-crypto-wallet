use anyhow::Result;
#[allow(unused_imports)]
use std::dbg;
mod eth_wallet;
mod utils;
mod tests;

use std::env;
// use eth_wallet::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    // println!("crypto_wallet: {:#?}", &crypto_wallet);
    crypto_wallet.save_to_file("./crypto_wallet.json")?;

    let loaded_wallet = eth_wallet::Wallet::from_file("./crypto_wallet.json")?;
    // println!("loaded_wallet: {:#?}", loaded_wallet);

    let endpoint = env::var("ALCHEMY_SEPOLIA_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = loaded_wallet.get_block(&web3_con).await?;
    dbg!(block_number);

    let balance = loaded_wallet.get_balance(&web3_con).await?;
    dbg!(balance);

    Ok(())
}


