#![feature(rustc_private)]

extern crate base58;
extern crate bitcoin;
extern crate exonum_bitcoinrpc;
extern crate rand;
extern crate serialize;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use exonum_bitcoinrpc::Client;
use bitcoin::blockdata::block::Block;
use bitcoin::network::serialize::{deserialize, serialize};
use serialize::hex::FromHex;
use base58::{FromBase58, ToBase58};

fn main() {
    println!("Urr!");
    let client = Client::new(
        "rpc://127.0.0.1:8332",
        Some("foo".to_string()),
        Some("bar".to_string()),
    );

    let block_count = client.getblockcount().unwrap();
    println!("Block count: {}", block_count);

    for height in block_count - 1..block_count {
        println!("Fetching {}", height);

        let block_hash = client.getblockhash(height).unwrap();
        println!("Block hash: {}", &block_hash);
        let block = client.getblock(block_hash).unwrap().from_hex().unwrap();
        println!("LEN {}", block.len());

        // println!("Block: {}", block);
        let block_bytes = block;
        let decode: Result<Block, _> = deserialize(&block_bytes);
        // assert!(decode.is_ok());
        let real_decode = decode.unwrap();
        // assert_eq!(real_decode.header.version, 1);
        println!("Header version {}", real_decode.header.version);
        println!("Transactions:");
        for tx in &real_decode.txdata {
            println!("  TX ID: {}", tx.txid());
            println!("     version: {}", tx.version);
            println!("     lock time: {}", tx.lock_time);
            println!("     Inputs:");
            for inp in &tx.input {
                println!("    INPUT prev-hash: {}", inp.prev_hash);
            }
            println!("     Outputs:");
            for out in &tx.output {
                println!("    OUTPUT val: {}", out.value);
                println!(
                    "           is_v0_p2wsh: {}",
                    out.script_pubkey.is_v0_p2wsh()
                );
                println!(
                    "           to_v0_p2wsh: {}",
                    out.script_pubkey.to_v0_p2wsh()
                );
                println!("           is_p2sh: {}", out.script_pubkey.is_p2sh());
                println!("           to_p2sh: {}", out.script_pubkey.to_p2sh());
                println!("           is_p2pkh: {}", out.script_pubkey.is_p2pkh());
                println!("           HEX: {:X}", out.script_pubkey);
            }
            println!("  TX =======");
        }
        println!("BLOCK ======");
        println!(
            "ADDR {:?}",
            String::from_utf8_lossy(&"1DYve98WqJQmx6cDdW2EQFpnqfwptjV4e9".from_base58().unwrap())
        );
    }
}
