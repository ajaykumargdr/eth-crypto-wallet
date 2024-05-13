mod account;
mod errors;
mod eth_wallet;
mod utils;

pub use account::*;
pub use errors::*;
pub use eth_wallet::*;
pub use utils::*;

use anyhow::{bail, Result};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, default, io::BufWriter, result};
use std::{fs::OpenOptions, io::BufReader};
use std::{
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
};
use tiny_keccak::keccak256;

use web3::{
    ethabi::Hash,
    futures::{StreamExt, TryStreamExt},
    transports::WebSocket,
    types::{
        Address, BlockId, BlockNumber, FilterBuilder, Log, Transaction, TransactionParameters,
        H256, U256, U64,
    },
    Web3,
};

