use std::path::PathBuf;

use super::*;
use crate::crypto::PBKDF2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    secret_key: String,
    public_address: String,
}

impl Account {
    pub fn create_new_account(derived_key: &PBKDF2) -> Result<Self> {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime);

        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let public_address = get_public_address(&public_key);

        let secret_key = hex::encode(derived_key.aes_encrypt(secret_key.to_string().as_bytes())?);

        Ok(Self {
            secret_key,
            public_address: format!("{public_address:?}"),
        })
    }

    pub fn new(secret_key: &str, derived_key: &PBKDF2) -> Result<Self> {
        let secret_key = SecretKey::from_str(secret_key)?;
        let secp = secp256k1::Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let public_address: Address = get_public_address(&public_key);

        let secret_key = hex::encode(derived_key.aes_encrypt(secret_key.to_string().as_bytes())?);

        Ok(Self {
            secret_key,
            public_address: format!("{:?}", public_address),
        })
    }

    pub fn get_secret(&self, derived_key: &PBKDF2) -> Result<SecretKey> {
        let key_hex = hex::decode(&self.secret_key)?;
        let secret_key = derived_key.aes_decrypt(&key_hex)?;

        let secret_key = std::str::from_utf8(&secret_key)?;

        Ok(SecretKey::from_str(secret_key)?)
    }

    pub fn get_address(&self) -> Result<Address> {
        Ok(Address::from_str(&self.public_address)?)
    }

    pub fn get_address_as_str(&self) -> String {
        self.public_address.clone()
    }

    pub fn export(&self, derived_key: &PBKDF2, path: &PathBuf, file_name: &str) -> Result<()> {
        let secret = derived_key.aes_decrypt(&hex::decode(&self.secret_key)?)?;

        let keys = serde_json::json!({ "secret_key": std::str::from_utf8(&secret)?, "public_address": self.public_address});

        std::fs::create_dir_all(path)?;
        let file = std::fs::File::create(path.join(file_name))?;
        serde_json::to_writer_pretty(file, &keys)?;

        Ok(())
    }

    pub async fn get_balance(&self, endpoint: &str) -> Result<f64> {
        let transport = web3::transports::Http::new(endpoint)?;
        let web3_client = web3::Web3::new(transport);

        let balance = web3_client.eth().balance(self.get_address()?, None).await?;

        Ok(utils::wei_to_eth(balance))
    }

    pub async fn make_transaction(
        &self,
        to: &str, // public address
        eth_value: f64,
        endpoint: &str,
        derived_key: &PBKDF2,
    ) -> Result<H256> {
        let transport = web3::transports::Http::new(endpoint)?;
        let web3_client = web3::Web3::new(transport);

        let address = Address::from_str(to)?;

        let transaction_params = TransactionParameters {
            to: Some(address),
            value: utils::eth_to_wei(eth_value),
            ..Default::default()
        };

        let signed_transaction = web3_client
            .accounts()
            .sign_transaction(transaction_params, &self.get_secret(derived_key)?)
            .await?;

        Ok(web3_client
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await?)
    }
}
