#[cfg(test)]
mod tests {
    use crate::eth_wallet::*;

    #[test]
    fn test_generate_keypair() {
        let (sk, pk) = generate_keypair();
        assert!(!sk.to_string().is_empty());
        assert!(!pk.to_string().is_empty());
    }

    #[test]
    fn test_public_key_address() {
        let (sk, pk) = generate_keypair();
        assert!(!sk.to_string().is_empty());
        assert!(!pk.to_string().is_empty());

        let public_address = public_key_address(&pk);
        assert!(!public_address.to_string().is_empty());
    }
}
