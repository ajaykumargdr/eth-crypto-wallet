use super::*;

#[derive(Debug)]
pub struct Wallet {
    accounts: HashMap<String, Account>,
    web3_client: Web3<web3::transports::Http>,
}

impl Wallet {
    pub async fn new(api_url: &str) -> Result<Self, Errors> {
        let transport = web3::transports::Http::new(api_url)
            .map_err(|error| Errors::ErrorCreatingWallet(error.to_string()))?;

        Ok(Self {
            accounts: HashMap::<String, Account>::new(),
            web3_client: web3::Web3::new(transport),
        })
    }

    pub fn get_account(&self, account_name: &str) -> Result<&Account, Errors> {
        match self.accounts.get(account_name) {
            Some(account) => Ok(account),
            None => Err(Errors::AccountDoesNotExist),
        }
    }

    pub fn create_and_add_account(&mut self, account_name: &str) -> Result<(), Errors> {
        if self.accounts.contains_key(account_name) {
            return Err(Errors::AccountAlreadyExist);
        }

        self.accounts
            .insert(account_name.to_string(), Account::create_new_account());

        Ok(())
    }

    pub fn add_account(
        &mut self,
        account_name: &str,
        secret_key: &str,
        public_key: &str,
    ) -> Result<(), Errors> {
        if self.accounts.contains_key(account_name) {
            return Err(Errors::AccountAlreadyExist);
        }

        self.accounts.insert(
            account_name.to_string(),
            Account::new(secret_key, public_key)?,
        );

        Ok(())
    }

    pub fn add_from_wallet_file(&mut self, account_name: &str, path: &str) -> Result<(), Errors> {
        if self.accounts.contains_key(account_name) {
            return Err(Errors::AccountAlreadyExist);
        }

        let account =
            Account::from_file(path).map_err(|err| Errors::ErrorInWalletFile(err.to_string()))?;

        self.accounts.insert(account_name.to_string(), account);

        Ok(())
    }

    pub fn save_to_file(&self, file_path: &str, account_name: &str) -> Result<(), Errors> {
        let account = self.get_account(account_name)?;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|err| Errors::ErrorCreatingWalletFile(err.to_string()))?;

        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, account)
            .map_err(|err| Errors::ErrorWritingWalletFile(err.to_string()))
    }

    pub fn get_secret_key(&self, account_name: &str) -> Result<SecretKey, Errors> {
        self.get_account(account_name)?.get_secret_key_typed()
    }

    pub fn get_public_key(&self, account_name: &str) -> Result<PublicKey, Errors> {
        self.get_account(account_name)?.get_public_key_typed()
    }

    pub fn get_address(&self, account_name: &str) -> Result<Address, Errors> {
        self.get_account(account_name)?.get_address()
    }

    pub async fn get_balance(&self, account_name: &str) -> Result<U256, Errors> {
        let account = self.get_account(account_name)?;

        self.web3_client
            .eth()
            .balance(account.get_address()?, None)
            .await
            .map_err(|error| Errors::ErrorGettingBalance(error.to_string()))
    }

    pub async fn get_balance_in_eth(&self, account_name: &str) -> Result<f64, Errors> {
        self.get_account(account_name)?;
        let balance = self.get_balance(account_name).await?;
        Ok(utils::wei_to_eth(balance))
    }

    pub async fn get_block_number(&self) -> Result<U64> {
        Ok(self.web3_client.eth().block_number().await?)
    }

    pub async fn make_transaction_from(
        &self,
        account_name: &str,
        to: &str,   // public address
        eth_value: f64,
    ) -> Result<H256, Errors> {
        let account = self.get_account(account_name)?;

        let address =
            Address::from_str(to).map_err(|error| Errors::ErrorInAddress(error.to_string()))?;

        let transaction_params = TransactionParameters {
            to: Some(address),
            value: utils::eth_to_wei(eth_value),
            ..Default::default()
        };

        let signed_transaction = self
            .web3_client
            .accounts()
            .sign_transaction(transaction_params, &account.get_secret_key_typed()?)
            .await
            .map_err(|error| Errors::ErrorSigningTransaction(error.to_string()))?;

        self.web3_client
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await
            .map_err(|error| Errors::ErrorMakingTransaction(error.to_string()))
    }
}
