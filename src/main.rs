mod block;
mod secp256;

use block::Block;
use secp256::Wallet;

fn main() {
    println!("Hello, world!");

    let mut block_chain = Vec::new();


    let mut difficulty = 1;
    let mut index = 0;

    let personal_wallet = Wallet::new();

    let block = Block::new("张帅".to_string(), "0".to_string(), index, difficulty);
    block_chain.push(block);

    loop {
        index += 1;
        difficulty += 1;

        let prev_block = block_chain.last().expect("Blockchain should have at least one block");
        let block = Block::new("张帅".to_string(), prev_block.hash.clone(), index, difficulty);
        block_chain.push(block);

        match difficulty {
            4 => {
                // 为了体现效果，这里的索引用困难程度代替
                let mined_block = block_chain.get(3).expect("Block should exist");
                println!("Block #{} has been mined: {} time consuming {}ms", mined_block.index, mined_block.hash, mined_block.get_time_to_generate_block());
                // println!("Hash length: {}", mined_block.hash.len()); // 打印哈希的长度
                let signature = personal_wallet.sign(mined_block.hash.clone());
                if personal_wallet.verify(personal_wallet.msg, signature, personal_wallet.public_key)
                {
                    println!("Signature is valid");
                } 
                else 
                {
                    println!("Signature is invalid");
                }
            }
            5 => {
                let mined_block = block_chain.get(4).expect("Block should exist");
                println!("Block #{} has been mined: {} time consuming {}ms", mined_block.index, mined_block.hash, mined_block.get_time_to_generate_block());
            }
            _ => {}
        }
        
    }

    
}
