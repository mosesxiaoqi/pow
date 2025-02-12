use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use hex;

// #[derive(Clone)]
pub struct Block{
    pub timestamp: u128,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub index: u64,
    pub nonce: u64,
    time_to_generate_block: u128,
}

impl Block{
    pub fn new(data: String, prev_hash: String, index: u64, difficulty: usize) -> Block
    {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let mut block = Block{
            timestamp,
            data,
            prev_hash,
            index,
            nonce: 0,
            hash: String::new(),
            time_to_generate_block: 0,
        };
        block.mine(difficulty);
        block.time_to_generate_block = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() - block.timestamp;
        block
    }

    fn calculate_hash(&self) -> String
    {
        let data = format!("{}{}{}{}{}", self.timestamp, self.data, self.prev_hash, self.index, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let res = hasher.finalize();
        hex::encode(res)
    }

    fn mine(& mut self, difficulty: usize)
    {
        let target = "0".repeat(difficulty);
        loop{
            self.nonce += 1;
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == target
            {
                break;
            }
        }

        println!("Block Mined: {}", self.hash);
    }

    pub fn get_time_to_generate_block(&self) -> u128
    {
        self.time_to_generate_block
    }
}