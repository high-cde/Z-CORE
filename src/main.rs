use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, prev_hash));
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }
}

fn genesis_block() -> Block {
    Block::new(0, "Z-CORE GENESIS BLOCK".to_string(), "0".to_string())
}

fn main() {
    println!("🚀 Z-CORE Node v0.1 avviato");

    let genesis = genesis_block();
    println!("🧬 Genesis Block: {:#?}", genesis);
}
