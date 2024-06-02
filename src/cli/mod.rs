use super::Result;
use crate::crypto;
use crate::wallet::Account;
use crate::{sha256, wallet};
use clap::{command, Subcommand};
pub use clap::{Args, Parser};
use spinoff::{spinners, Color, Spinner};
use std::path::PathBuf;

mod commands;
mod handlers;

pub use commands::*;

const ASCII_LOGO: &str = r#"

  /$$$$$$                      /$$      /$$           /$$ /$$             /$$
 /$$__  $$                    | $$  /$ | $$          | $$| $$            | $$    
| $$  \__/  /$$$$$$  /$$$$$$$ | $$ /$$$| $$  /$$$$$$ | $$| $$  /$$$$$$  /$$$$$$  
| $$ /$$$$ /$$__  $$| $$__  $$| $$/$$ $$ $$ |____  $$| $$| $$ /$$__  $$|_  $$_/  
| $$|_  $$| $$$$$$$$| $$  \ $$| $$$$_  $$$$  /$$$$$$$| $$| $$| $$$$$$$$  | $$    
| $$  \ $$| $$_____/| $$  | $$| $$$/ \  $$$ /$$__  $$| $$| $$| $$_____/  | $$ /$$
|  $$$$$$/|  $$$$$$$| $$  | $$| $$/   \  $$|  $$$$$$$| $$| $$|  $$$$$$$  |  $$$$/
 \______/  \_______/|__/  |__/|__/     \__/ \_______/|__/|__/ \_______/   \___/
"#;

#[derive(Debug, Parser)]
#[command(
    name = "genwallet",
    version = "0.0.1",
    before_help = ASCII_LOGO,
    long_about = "GenWallet (Genesis Wallet) is a powerful yet user-friendly command-line interface (CLI),
that empowers you to manage your Ethereum accounts directly from your terminal.
Genwallet provides a comprehensive set of features to handle various account-related
operations securely and efficiently."
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        self.commands.run().await
    }
}
