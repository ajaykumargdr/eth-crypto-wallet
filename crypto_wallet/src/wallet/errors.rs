use super::*;
use std::io::Error;

#[derive(Debug)]
pub enum Errors{
    Error(String),
    ErrorCreatingWallet(String),
    AccountAlreadyExist,
    AccountDoesNotExist,
    ErrorInWalletFile(String),
    ErrorCreatingWalletFile(String),
    ErrorWritingWalletFile(String),
    ErrorCreatingSecretKey(String),
    ErrorCreatingPublicKey(String),
    ErrorMakingTransaction(String),
    ErrorSigningTransaction(String),
    ErrorInAddress(String),
    ErrorGettingBalance(String),
}