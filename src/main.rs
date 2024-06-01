mod crypto;
mod wallet;

mod cli;
pub use cli::*;
pub use crypto::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let rslt = Cli::parse().run().await;

    if rslt.is_err() {
        println!("something went wrong!");
    }

    Ok(())
}
