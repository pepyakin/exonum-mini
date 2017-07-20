#[macro_use]
extern crate exonum_mini;

extern crate serde_json;

use exonum_mini::crypto;
use crypto::{PublicKey, Seed};

const SERVICE_ID: u16 = 1;
const TX_CREATE_WALLET_ID: u16 = 1;
const TX_TRANSFER_ID: u16 = 2;

encoding_struct! {
    struct Wallet {
        const SIZE = 48;

        field pub_key:            &PublicKey  [00 => 32]
        field name:               &str        [32 => 40]
        field balance:            u64         [40 => 48]
    }
}

message! {
    struct TxCreateWallet {
        const TYPE = SERVICE_ID;
        const ID = TX_CREATE_WALLET_ID;
        const SIZE = 40;

        field pub_key:     &PublicKey  [00 => 32]
        field name:        &str        [32 => 40]
    }
}

message! {
    struct TxTransfer {
        const TYPE = SERVICE_ID;
        const ID = TX_TRANSFER_ID;
        const SIZE = 80;

        field from:        &PublicKey  [00 => 32]
        field to:          &PublicKey  [32 => 64]
        field amount:      u64         [64 => 72]
        field seed:        u64         [72 => 80]
    }
}

fn main() {
    crypto::init();
    let (public_key, secret_key) = crypto::gen_keypair_from_seed(&Seed::new([1; 32]));

    let create_wallet_tx = TxCreateWallet::new(&public_key, "John", &secret_key);
    let create_wallet_tx_json = serde_json::to_string(&create_wallet_tx).unwrap();
    println!("json={}", create_wallet_tx_json);
}
