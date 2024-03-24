use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    secret_key: String,
    public_key: String,
    public_address: String,
}

impl Account {
    pub fn create_new_account() -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime);

        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let public_address = get_public_address(&public_key);

        Self {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{public_address:?}"),
        }
    }

    pub fn new(secret_key: &str, public_key: &str) -> Result<Self, Errors> {
        let secret_key = match SecretKey::from_str(secret_key) {
            Ok(sk) => sk,
            Err(error) => return Err(Errors::ErrorCreatingSecretKey(error.to_string())),
        };

        let public_key = match PublicKey::from_str(public_key) {
            Ok(pk) => pk,
            Err(error) => return Err(Errors::ErrorCreatingPublicKey(error.to_string())),
        };

        let addr: Address = get_public_address(&public_key);

        Ok(Self {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        })
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);
        let account: Self = serde_json::from_reader(buf_reader)?;
        Ok(account)
    }

    pub fn get_secret_key_typed(&self) -> Result<SecretKey, Errors> {
        match SecretKey::from_str(&self.secret_key) {
            Ok(sk) => Ok(sk),
            Err(error) => return Err(Errors::ErrorCreatingSecretKey(error.to_string())),
        }
    }

    pub fn get_public_key_typed(&self) -> Result<PublicKey, Errors> {
        match PublicKey::from_str(&self.public_key) {
            Ok(pk) => Ok(pk),
            Err(error) => return Err(Errors::ErrorCreatingPublicKey(error.to_string())),
        }
    }

    pub fn get_address(&self) -> Result<Address, Errors> {
        match Address::from_str(&self.public_address){
            Ok(add) => Ok(add),
            Err(error) => return Err(Errors::ErrorInAddress(error.to_string()))
        }
    }



}
