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

#[derive(Debug, Parser)]
#[command(
    name = "genwallet",
    version = "0.0.1",
    long_about = "Genesis Wallet (genwallet) is a powerful yet user-friendly command-line interface (CLI) that empowers you to manage your Ethereum accounts directly from your terminal."
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
