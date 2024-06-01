use super::Result;

use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

mod pbkdf2_aes;

#[cfg(test)]
mod tests;

pub use pbkdf2_aes::*;
