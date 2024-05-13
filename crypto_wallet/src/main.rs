use anyhow::Result;

mod wallet;
pub use wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_endpoint = std::env::var("ALCHEMY_SEPOLIA_HTTP")?;
    let mut wallet = Wallet::new(&api_endpoint).await.unwrap();

    let block = wallet.get_block_number().await.unwrap();
    println!("block number: {}", block);

    // creating a new account and adding it to the genwallet
    wallet.create_and_add_account("my_first_wallet").unwrap();

    // getting balance from an account
    let balance = wallet.get_balance_in_eth("my_first_wallet").await.unwrap();
    assert_eq!(balance, 0.0);

    // importing an account by taking account credentials from a file
    wallet
        .add_from_wallet_file("genesis_wallet", "./crypto_wallet-prv.json")
        .unwrap();

    // getting balance from an account
    let balance = wallet.get_balance_in_eth("genesis_wallet").await.unwrap();
    println!("balance of genesis wallet: {}", balance);

    // getting account details
    let _account = wallet.get_account("genesis_wallet").unwrap();

    // making a transaction
    let tx = wallet
        .make_transaction_from(
            "genesis_wallet",
            "0x188bD975BFdf02131b20B0219A5B89B2293c1BAd",
            0.0001,
        )
        .await
        .unwrap();

    println!("transaction: {}", tx);

    Ok(())
}
