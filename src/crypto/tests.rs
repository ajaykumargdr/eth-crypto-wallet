#[test]
fn test_sha256() {
    use hex;
    let data = super::sha256(b"hello world");

    assert_eq!(
        Ok(data),
        hex::decode("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    );
}

#[test]
fn test_pbkdf2() {
    let derived_key = super::PBKDF2::new(b"genwallet");
    let data = "ba9acecc8a2ef6999d00878f5dde2631010243ad765823d011417a1a4b1ac320";

    let enc_data = derived_key.aes_encrypt(data.as_bytes()).unwrap();
    let dec_data = derived_key.aes_decrypt(&enc_data).unwrap();

    assert_eq!(dec_data, data.as_bytes());
}
