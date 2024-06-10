![Genesis wallet logo](./logo/Genesis%20wallet%20logo.png)
# Genwallet (Genesis Wallet)

Genwallet is a powerful yet user-friendly command-line interface (CLI) for managing your Ethereum accounts directly from your terminal. Genwallet provides a comprehensive set of features to handle various account-related operations securely and efficiently.

## Table of Contents

- [Features](#features)
- [Security](#security)
- [Install Rust](#install-rust)
- [Install Genwallet](#install-genwallet)
- [Usage](#usage)
  - [Initialize the Wallet](#initialize-the-wallet)
  - [Create a New Account](#create-a-new-account)
  - [Import an Existing Account](#import-an-existing-account)
  - [Export an Account to a File](#export-an-account-to-a-file)
  - [Get Account Balance](#get-account-balance)
  - [Get the Public Address of an Account](#get-the-public-address-of-an-account)
  - [Transfer ETH Between Accounts](#transfer-eth-between-accounts)
  - [List All Accounts](#list-all-accounts)
  - [Rename an Account](#rename-an-account)
  - [Remove an Account](#remove-an-account)
- [Internet Access](#internet-access)
- [Contributing](#contributing)
- [License](#license)
- [Questions and Feedback](#questions-and-feedback)

## Features

- Create new accounts
- Import existing accounts
- Export accounts to files
- Retrieve account balances
- Get public addresses of accounts
- Execute ETH transfers between accounts
- List all accounts with public addresses and balances
- Rename accounts
- Remove accounts

## Security

Genwallet employs strong cryptographic algorithms to ensure the security of your passwords and secret keys. The following algorithms are used:

- **PBKDF2**: Used to derive cryptographic keys from the user-provided password.
- **SHA-256**: Utilized to create an authentication key for the password.
- **AES-256-CBC**: Used to encrypt sensitive data such as endpoints and account credentials.

Note: While Genwallet uses robust cryptographic algorithms, the strength of the protection depends on the strength of your password.

## Install Rust

Genwallet requires Rust to be installed on your system. Follow the steps below to install Rust:

1. Go to the [Rust installation page](https://www.rust-lang.org/tools/install).
2. Follow the instructions to install Rust using `rustup`.

Alternatively, you can run the following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After the installation is complete, ensure Rust is added to your system's PATH and verify the installation by checking the version:

```sh
rustc --version
```

## Install Genwallet

To install Genwallet, run the following command:

```sh
cargo install --git https://github.com/ajaykumargdr/eth-crypto-wallet.git
```

After the installation, you can run the Genwallet CLI using the following command:

```sh
genwallet --help
```

If you see the following output, it means that Genwallet is successfully installed on your system:

![genwallet --help output](./logo/genwallet%20installation.png)

## Usage

Genwallet provides a variety of commands to manage your Ethereum accounts. Below is a detailed guide on how to use each command.

### Initialize the Wallet

```sh
genwallet init <endpoint>
```

Example:

```sh
genwallet init https://eth-sepolia.g.alchemy.com/v2/8qXsAot0Z9IQBouKAxAjZgsrPToTSkiE
```
Create your endpoint from [Alchemy](https://dashboard.alchemy.com/apps). You can also other providers as well.
### Create a New Account

```sh
genwallet create <account_name>
```

Example:

```sh
genwallet create my_account
```

### Import an Existing Account

```sh
genwallet import <account_name> --file <path_to_keyfile>
genwallet import <account_name> --key <secret_key>
```

Examples:

```sh
genwallet import my_account --file /path/to/secret.key
genwallet import my_account --key ba9axxxx8a2ef6xxxd00878f5xxx273101024xxx765823d0114xxxxa4b1ac32
```

### Export an Account to a File

```sh
genwallet export <account_name> <path>
```

Example:

```sh
genwallet export my_account /path/to/exported.key
```

### Get Account Balance

```sh
genwallet balance <account_name>
```

Example:

```sh
genwallet balance my_account
```

### Get the Public Address of an Account

```sh
genwallet id <account_name>
```

Example:

```sh
genwallet id my_account
```

### Transfer ETH Between Accounts

```sh
genwallet transfer <account_name> <to> <eth>
```

Example:

```sh
genwallet transfer my_account 0xRecipientAddress 0.1
```

### List All Accounts

```sh
genwallet list  # list without balances
genwallet list -b  # list with balances
```

Example:

```sh
genwallet list -b
```

### Rename an Account

```sh
genwallet rename <old_account_name> <new_account_name>
```

Example:

```sh
genwallet rename old_name new_name
```

### Remove an Account

```sh
genwallet remove <account_name>
```

Example:

```sh
genwallet remove my_account
```

## Internet Access

Genwallet requires an internet connection for the following operations:
- Initializing the wallet (`init`)
- Retrieving account balances (`balance`)
- Listing all accounts with their balances (`list --balance`)
- Executing transactions (`transfer`)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes or additions you'd like to see.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Questions and Feedback

For questions, feedback, or support, please open an issue on this GitHub repository.

---
