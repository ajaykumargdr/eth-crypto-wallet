use super::*;

pub struct PBKDF2 {
    key: [u8; 80],
}

impl PBKDF2 {
    pub fn new(password: &[u8]) -> Self {
        let key = pbkdf2_hmac_array::<Sha256, 80>(password, b"-1", 50_000);
        Self { key }
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    pub fn get_iv(&self) -> &[u8] {
        &self.key[0..16]
    }

    pub fn get_cipher_key(&self) -> &[u8] {
        &self.key[16..48]
    }

    pub fn get_mac_key(&self) -> &[u8] {
        &self.key[48..80]
    }

    pub fn aes_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Cbc::new_from_slices(&self.key[16..48], &self.key[0..16])?;

        let mut data = data.to_vec();
        let pos = data.len();
        data.append(&mut vec![0u8; 20]);

        Ok(cipher.encrypt(&mut data, pos)?.to_vec())
    }

    pub fn aes_decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Cbc::new_from_slices(&self.key[16..48], &self.key[0..16])?;

        let mut buf = cipher_text.to_vec();
        Ok(cipher.decrypt(&mut buf)?.to_vec())
    }
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
