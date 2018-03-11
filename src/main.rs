#[macro_use]
extern crate serde_json;
extern crate crypto;
extern crate time;

use std::thread;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::SystemTime;
use std::time::Duration;


struct Block {
    index: i64,
    previous_hash: String,
    timestamp: i64,
    data: String,
    hash: String,
    nonce: i64,
}

struct Blockchain {
    total_transactions: u64,
    inception: u64,
    blocks: Vec<Block>
}

struct BlockData {
	i: String,
	b: i32
}

impl Blockchain {
    fn add_block(&mut self, block: Block) {
        self.blocks.push(block)
    }
    fn increase_transaction(&mut self) {
        self.total_transactions = self.total_transactions + 1;
    }
}

fn is_prime(x: u32) -> bool{
    for i in 2..x{
        if x%i == 0 {
            return false;
        }
    }
    true
}

fn calculate_hash_for_block(index: i32, previous_hash: &str, timestamp: u64, data: String, nonce: i32) -> String {
    let input = index.to_string() + &previous_hash + &timestamp.to_string() + &data + &nonce.to_string();
    let mut sha = Sha256::new();
    sha.input_str(&input);
    // chain.total_transactions = chain.total_transactions + 1;
    sha.result_str()
}

fn get_genesis() -> Block {
    let block = Block { index: 0, previous_hash: "0".to_string(), timestamp: time::get_time().sec, data: "{ name: 'Genesis Block', value: 0 }".to_string(), hash: "1d79e9eef321cac0aa8f73d1245a5604a8a665e6daacf64d1b9843e2ab98fa29".to_string(), nonce: 745};
    block
}

fn is_valid_hash_difficulty(hash: &str) -> bool {
    let mut totalCount: u32 = 0;
    let mut charACount = 0;
    let a: &str = "a";

    for c in hash.chars() {
        if c.to_string() == a {
            charACount = charACount + 1;
        }
        let number_from_hash = c.to_digit(10);
        if number_from_hash != None {
            totalCount = totalCount + number_from_hash.unwrap();
        }
    }
    let isValid = is_prime(totalCount) && charACount >= 10;
    isValid
}

fn generate_next_block(ref mut chain: &mut Blockchain, data: BlockData) -> Block {
    let len = chain.blocks.len() - 1;
    let latest_block = &chain.blocks[len];
    let next_index = latest_block.index + 1;
    let previous_hash = &latest_block.previous_hash;
    let mut timestamp = time::get_time().sec;
    let mut nonce = 0;
    let block_payload = json!({
        "first_thing": data.i,
        "second_thing": data.b,
    });
    let mut nextHash = calculate_hash_for_block(next_index as i32, previous_hash, timestamp as u64, block_payload.to_string(), nonce);
    chain.total_transactions = chain.total_transactions + 1;
    while !is_valid_hash_difficulty(&nextHash) {
        nonce = nonce + 1;
        timestamp = time::get_time().sec;
        nextHash = calculate_hash_for_block(next_index as i32, previous_hash, timestamp as u64, block_payload.to_string(), nonce);
    }

    Block{index: next_index, previous_hash: previous_hash.to_string(), timestamp: timestamp, data: block_payload.to_string(), hash: nextHash.to_string(), nonce: nonce as i64}
}

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn main() {
    let mut blocks = Vec::new();
    blocks.push(get_genesis());
    let mut chain = Blockchain{total_transactions: 0, inception: time::get_time().sec as u64, blocks: blocks};

    let chainToMine: Vec<BlockData> = vec![
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 3".to_string(), b: 3 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
        BlockData{ i: "test 2".to_string(), b: 2 },
        BlockData{ i: "test 4".to_string(), b: 4 },
        BlockData{ i: "test 1".to_string(), b: 1 },
    ];
    let start = timestamp();
    for c in chainToMine {
        let newBlock = generate_next_block(&mut chain, c);
        chain.add_block(newBlock);
    }
    println!("{:?} transactions", chain.total_transactions);
    println!("total time {:?}d", timestamp() - start)
}
