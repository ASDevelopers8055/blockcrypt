//! Blockchain Crypt
//! A library for making a PoW blockchain with minimal functionality.

///
/// # Example
/// 
/// ```
/// use blockcrypt::*;

/// fn main () {
///     let difficulty = 0x000fffffffffffffffffffffffffffff;
///
///     let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
///         Transaction {
///             inputs: vec![ ],
///             outputs: vec![
///                 transaction::Output {
///                     to_addr: "Alice".to_owned(),
///                     value: 50,
///                 },
///                 transaction::Output {
///                     to_addr: "Bob".to_owned(),
///                     value: 7,
///                 },
///             ],
///         },
///     ], difficulty);
///
///     genesis_block.mine();
///
///     println!("Mined genesis block {:?}", &genesis_block);
///
///     let mut last_hash = genesis_block.hash.clone();
///
///     let mut blockchain = Blockchain::new();
///
///     blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
///
///     let mut block = Block::new(1, now(), last_hash, vec![
///         Transaction {
///             inputs: vec![ ],
///             outputs: vec![
///                 transaction::Output {
///                     to_addr: "Chris".to_owned(),
///                     value: 536,
///                 },
///             ],
///         },
///         Transaction {
///             inputs: vec![
///                 blockchain.blocks[0].transactions[0].outputs[0].clone(),
///             ],
///             outputs: vec![
///                 transaction::Output {
///                     to_addr: "Alice".to_owned(),
///                     value: 360,
///                 },
///                 transaction::Output {
///                     to_addr: "Bob".to_owned(),
///                     value: 12,
///                 },
///             ],
///         },
///     ], difficulty);
///
///     block.mine();
///
///     println!("Mined block {:?}", &block);
///
///     last_hash = block.hash.clone();
///
///     blockchain.update_with_block(block).expect("Failed to add block");
/// }
/// ```

mod block;
mod blockchain;
mod hash;
pub mod transaction;

pub use crate::block::Block;
pub use crate::hash::Hash;
pub use crate::blockchain::Blockchain;
pub use crate::transaction::Transaction;

use std::time::{ SystemTime, UNIX_EPOCH };

/// Returns the current time in seconds since the UNIX epoch.
pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
    ;

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

/// Converts the given 32 bit unsigned integer into bytes
pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

/// Converts the given 64 bit unsigned integer into bytes
pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

/// Converts the given 128 bit unsigned integer into bytes
pub fn u128_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,

        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,

        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

/// Converts the difficulty into bytes
pub fn difficulty_bytes_as_u128 (v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8) |
    ((v[30] as u128) << 0xe * 8) |
    ((v[29] as u128) << 0xd * 8) |
    ((v[28] as u128) << 0xc * 8) |
    ((v[27] as u128) << 0xb * 8) |
    ((v[26] as u128) << 0xa * 8) |
    ((v[25] as u128) << 0x9 * 8) |
    ((v[24] as u128) << 0x8 * 8) |
    ((v[23] as u128) << 0x7 * 8) |
    ((v[22] as u128) << 0x6 * 8) |
    ((v[21] as u128) << 0x5 * 8) |
    ((v[20] as u128) << 0x4 * 8) |
    ((v[19] as u128) << 0x3 * 8) |
    ((v[18] as u128) << 0x2 * 8) |
    ((v[17] as u128) << 0x1 * 8) |
    ((v[16] as u128) << 0x0 * 8)
}
