use super::*;

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Todo(return secret phrase)
    #[command(about = "Creates/Initialize the wallet")]
    Init {
        /// The endpoint to connect to
        endpoint: String,
    },

    #[command(about = "Creates a new account")]
    Create {
        /// A new name for the account to be created
        account_name: String,
    },

    #[command(about = "Imports an existing account")]
    Import {
        /// A new name for the account to be imported
        account_name: String,

        /// The secret key of the account to be imported
        #[command(subcommand)]
        secret: SecretKey,
    },

    #[command(about = "Exports an existing account to a file")]
    Export {
        /// The name of the account to be exported
        account_name: String,

        /// The path to export the account to
        path: PathBuf,
    },

    #[command(about = "Gets the balance of an account")]
    Balance {
        /// The name of the account to get the balance of
        account_name: String,
    },

    #[command(about = "Gets the balance of an account")]
    Id {
        /// The name of the account to get the public address of
        account_name: String,
    },

    #[command(about = "Makes a new transaction to an account")]
    Transfer {
        /// The name of the account to transfer from
        account_name: String,

        /// Public address of the account to transfer to
        to: String,

        /// Amount of ETH to transfer
        eth: f64,
    },

    #[command(about = "Lists all account names along with their public addresses")]
    List {
        /// Get the balance of all accounts
        #[arg(long, short, action)]
        balance: bool,
    },

    #[command(about = "Renames a account name")]
    Rename {
        /// The name of the account to be renamed
        old_account_name: String,

        /// The new name of the account
        new_account_name: String,
    },

    #[command(about = "Renames a account name")]
    Remove {
        /// The name of the account to be removed
        account_name: String,
    },
}

#[derive(Debug, Clone, Parser)]
pub enum SecretKey {
    #[command(about = "Import from file")]
    File {
        /// The path to the secret key file
        path: PathBuf,
    },
    #[command(about = "Import from secret key")]
    Key {
        /// The secret key to import
        secret_key: String,
    },
}

impl Commands {
    pub async fn run(&self) -> Result<()> {
        match &self {
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
