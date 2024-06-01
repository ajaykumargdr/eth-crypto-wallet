use super::*;

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
    pub async fn run(self) -> Result<()> {
        match &self.commands {
            Commands::Init { endpoint } => handlers::init(endpoint).await,

            Commands::Create { account_name } => handlers::create(account_name),

            Commands::Import {
                account_name,
                secret,
            } => handlers::import(account_name, secret),

            Commands::Export { account_name, path } => handlers::export(account_name, path),

            Commands::Balance { account_name } => handlers::balance(account_name).await,

            Commands::Id { account_name } => handlers::id(account_name),

            Commands::Transfer {
                account_name,
                to,
                eth,
            } => handlers::transfer(account_name, to, *eth).await,

            Commands::List { balance } => handlers::list(*balance).await,

            Commands::Rename {
                old_account_name,
                new_account_name,
            } => handlers::rename(old_account_name, new_account_name),

            Commands::Remove { account_name } => handlers::remove(account_name),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Todo(return secret phrase)
    #[command(about = "Creates/Initialize the wallet")]
    Init { endpoint: String },

    #[command(about = "Creates a new account")]
    Create { account_name: String },

    #[command(about = "Imports an existing account")]
    Import {
        account_name: String,

        #[command(subcommand)]
        secret: SecretKey,
    },

    #[command(about = "Exports an existing account to a file")]
    Export { account_name: String, path: PathBuf },

    #[command(about = "Gets the balance of an account")]
    Balance { account_name: String },

    #[command(about = "Gets the balance of an account")]
    Id { account_name: String },

    #[command(about = "Makes a new transaction to an account")]
    Transfer {
        account_name: String,
        to: String,
        eth: f64,
    },

    #[command(about = "Lists all account names along with their public addresses")]
    List {
        #[arg(long, short, action)]
        balance: bool,
    },

    #[command(about = "Renames a account name")]
    Rename {
        old_account_name: String,
        new_account_name: String,
    },

    #[command(about = "Renames a account name")]
    Remove { account_name: String },
}

#[derive(Debug, Clone, Parser)]
pub enum SecretKey {
    #[command(about = "Import from file")]
    File { path: PathBuf },
    #[command(about = "Import from secret key")]
    Key { secret_key: String },
}
