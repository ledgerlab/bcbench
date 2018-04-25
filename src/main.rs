#![feature(rustc_private)]

extern crate exonum_bitcoinrpc;
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use exonum_bitcoinrpc::Client;

fn main() {
    println!("Urr!");
    let client = Client::new(
        "rpc://127.0.0.1:8332",
        Some("foo".to_string()),
        Some("bar".to_string()),
    );

    let block_count = client.getblockcount().unwrap();

    println!("Block count: {}", block_count);
}
