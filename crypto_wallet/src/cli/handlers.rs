use super::*;

fn get_db_path() -> PathBuf {
    dirs::home_dir()
        .expect("home directory path not found on $HOME")
        .join(".genwallet")
}

pub async fn init(endpoint: &str) -> Result<()> {
    let db_path = get_db_path();
    let transport = web3::transports::Http::new(endpoint)?;
    let web3 = web3::Web3::new(transport);

    if web3.eth().block_number().await.is_ok() {
        println!("endpoint accepted");
    }

    let init_password = rpassword::prompt_password("Enter password: ")?;
    let confirm_password = rpassword::prompt_password("Confirm password: ")?;

    if init_password != confirm_password {
        println!("passwords are same!");
        return Ok(());
    }

    let db = sled::open(db_path).expect("error in database!");

    let derived_key = crypto::PBKDF2::new(init_password.as_bytes());

    db.insert("user_password", crypto::sha256(derived_key.get_key()))?;
    db.insert("endpoint", derived_key.aes_encrypt(endpoint.as_bytes())?)?;

    println!("wallet successfully initialized💃");
    Ok(())
}

pub fn create(account_name: &str) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let db = db.open_tree("keys")?;

        if db.get(account_name)?.is_some() {
            println!("account name already exist!");
            println!("user different account_name");
            return Ok(());
        }

        let password = rpassword::prompt_password("Enter your password: ")?;

        let derived_key = crypto::PBKDF2::new(password.as_bytes());

        if sha256(derived_key.get_key()) != *user_password {
            println!("incorrect password!");
            println!("try again");
            return Ok(());
        }

        let account = wallet::Account::create_new_account(&derived_key)?;

        let account = serde_json::to_vec(&account)?;
        db.insert(account_name, account)?;
        println!("account created");
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub fn import(account_name: &str, secret: &SecretKey) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let db = db.open_tree("keys")?;

        if db.get(account_name)?.is_some() {
            println!("account name already exist!");
            println!("user different account_name");
            return Ok(());
        }

        let secret_key = match secret {
            SecretKey::File { path } => {
                let file = match std::fs::File::open(path) {
                    Ok(file) => file,
                    Err(_) => {
                        println!("secret key file does not exist!");
                        return Ok(());
                    }
                };

                let val: serde_json::Value = serde_json::from_reader(file)?;
                let secret_key = val.get("secret_key");

                match secret_key {
                    Some(secret_key) => serde_json::from_value::<String>(secret_key.clone())?,
                    None => {
                        println!("secret key not found in the file!");
                        return Ok(());
                    }
                }
            }
            SecretKey::Key { secret_key } => secret_key.clone(),
        };

        let password = rpassword::prompt_password("Enter your password: ")?;
        let derived_key = crypto::PBKDF2::new(password.as_bytes());

        if sha256(derived_key.get_key()) != *user_password {
            println!("incorrect password!");
            println!("try again");
            return Ok(());
        }

        let account = match wallet::Account::new(&secret_key, &derived_key) {
            Ok(account) => account,
            Err(_) => {
                println!("error in given secretekey!");
                return Ok(());
            }
        };

        let account = serde_json::to_vec(&account)?;
        db.insert(account_name, account)?;
        println!("account imported");
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub fn export(account_name: &str, path: &PathBuf) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let db = db.open_tree("keys")?;

        match db.get(account_name)? {
            Some(account) => {
                let password = rpassword::prompt_password("Enter your password: ")?;
                let derived_key = crypto::PBKDF2::new(password.as_bytes());

                if sha256(derived_key.get_key()) != *user_password {
                    println!("incorrect password!");
                    println!("try again");
                    return Ok(());
                }

                let account: Account = serde_json::from_slice(&account)?;
                account.export(&derived_key, path, &format!("{account_name}_keys.json"))?;
                println!(
                    "account exported to {:?}",
                    path.join(format!("{account_name}_keys.json"))
                );
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub async fn balance(account_name: &str) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let key_db = db.open_tree("keys")?;

        match key_db.get(account_name)? {
            Some(account) => {
                let account: Account = serde_json::from_slice(&account)?;

                let password = rpassword::prompt_password("Enter your password: ")?;
                let derived_key = crypto::PBKDF2::new(password.as_bytes());

                if sha256(derived_key.get_key()) != *user_password {
                    println!("incorrect password!");
                    println!("try again");
                    return Ok(());
                }

                let endpoint = db.get("endpoint")?;

                if let Some(endpoint) = endpoint {
                    let endpoint = derived_key.aes_decrypt(&endpoint)?;

                    let mut spinner = Spinner::new(spinners::Dots, "Loading...", Color::Blue);

                    match account.get_balance(std::str::from_utf8(&endpoint)?).await {
                        Ok(balance) => spinner.success(&format!("Balance: {balance}")),
                        Err(_) => println!("not able to find balance!"),
                    }
                } else {
                    println!("endpoint not found in the db!");
                    println!("please reinitialize the genwallet using the same password!");
                }
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub fn id(account_name: &str) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if user_password.is_some() {
        let key_db = db.open_tree("keys")?;

        match key_db.get(account_name)? {
            Some(account) => {
                let account: Account = serde_json::from_slice(&account)?;

                println!("Public Id: {}", account.get_address_as_str());
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub async fn transfer(account_name: &str, to: &str, eth: f64) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let key_db = db.open_tree("keys")?;

        match key_db.get(account_name)? {
            Some(account) => {
                let account: Account = serde_json::from_slice(&account)?;

                let password = rpassword::prompt_password("Enter your password: ")?;

                let derived_key = crypto::PBKDF2::new(password.as_bytes());

                if sha256(derived_key.get_key()) != *user_password {
                    println!("incorrect password!");
                    println!("try again");
                    return Ok(());
                }

                let endpoint = db.get("endpoint")?;

                if let Some(endpoint) = endpoint {
                    let mut spinner =
                        Spinner::new(spinners::Aesthetic, "Speeding up...", Color::Red);

                    let endpoint = derived_key.aes_decrypt(&endpoint)?;
                    let endpoint = std::str::from_utf8(&endpoint)?;

                    if account
                        .make_transaction(to, eth, endpoint, &derived_key)
                        .await
                        .is_err()
                    {
                        println!("not able to transfer eth!");
                        println!("please try again");
                    }

                    spinner.success("Transaction Complete");
                } else {
                    println!("endpoint not found in the db!");
                    println!("please reinitialize the genwallet using the same password!");
                }
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}
pub async fn list(balance: bool) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let key_db = db.open_tree("keys")?;

        let endpoint = if balance {
            let password = rpassword::prompt_password("Enter your password: ")?;
            let derived_key = crypto::PBKDF2::new(password.as_bytes());

            if sha256(derived_key.get_key()) != *user_password {
                println!("incorrect password!");
                println!("try again");
                return Ok(());
            }

            let endpoint = db.get("endpoint")?;

            if let Some(endpoint) = endpoint {
                let endpoint = derived_key.aes_decrypt(&endpoint)?;

                Some(std::str::from_utf8(&endpoint)?.to_string())
            } else {
                None
            }
        } else {
            None
        };

        for account in key_db.iter().flatten() {
            let name = String::from_utf8(account.0.to_vec())?;
            let account: Account = serde_json::from_slice(&account.1)?;

            println!("Name: '{name}' Id: {}", account.get_address_as_str());

            if balance {
                if let Some(endpoint) = &endpoint {
                    let mut spinner =
                        Spinner::new(spinners::Dots, "Checking balance...", Color::Blue);

                    match account.get_balance(endpoint).await {
                        Ok(balance) => {
                            spinner.success(&format!("Balance: {balance}\n"));
                        }
                        Err(_) => {
                            spinner.fail("Failed");
                            println!("Not able to find balance for the account: {}!", name);
                        }
                    }
                } else {
                    println!("endpoint not found in the db!");
                    println!("please reinitialize the genwallet using the same password!");
                    return Ok(());
                };
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub fn rename(old_account_name: &str, new_account_name: &str) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let key_db = db.open_tree("keys")?;

        match key_db.get(old_account_name)? {
            Some(account) => {
                let password = rpassword::prompt_password("Enter your password: ")?;
                let derived_key = crypto::PBKDF2::new(password.as_bytes());

                if sha256(derived_key.get_key()) != *user_password {
                    println!("incorrect password!");
                    println!("try again");
                    return Ok(());
                }

                key_db.remove(old_account_name)?;
                key_db.insert(new_account_name, account)?;
                println!("account renamed");
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}

pub fn remove(account_name: &str) -> Result<()> {
    let db_path = get_db_path();
    let db = sled::open(db_path)?;
    let user_password = db.get("user_password")?;

    if let Some(user_password) = user_password {
        let key_db = db.open_tree("keys")?;

        match key_db.get(account_name)? {
            Some(_) => {
                let password = rpassword::prompt_password("Enter your password: ")?;
                let derived_key = crypto::PBKDF2::new(password.as_bytes());

                if sha256(derived_key.get_key()) != *user_password {
                    println!("incorrect password!");
                    println!("try again");
                    return Ok(());
                }

                key_db.remove(account_name)?;
                println!("account removed");
            }
            None => {
                println!("account name does not exist!");
                return Ok(());
            }
        }
    } else {
        println!("wallet is not initialized!");
        println!("please initialize the wallet!");
        println!("use `genwallet init`");
    }

    Ok(())
}
