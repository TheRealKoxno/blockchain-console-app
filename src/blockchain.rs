extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use indicatif::ProgressBar;

use self::sha2::{Sha256, Digest};
use std::{fmt::Write};

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: SystemTime,
    nonce: u32, //number of attempts taken to find the block
    pre_hash: String, //previous hash
    merkle: String,
    difficulty: u32, //the difficulty of finding blocks in our network
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32, //the number of transactions in the stored block
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>, //vector of blocks validated by the miners
    curr_transactions: Vec<Transaction>, //transactions that didn't fall into any block and and will be validated by miners, when mining the next block
    difficulty: u32, //current network difficulty
    miner_address: String, //miner address for rewarding
    reward: f32,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transactions: Vec::new(),
            difficulty,
            miner_address,
            reward: 50.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_transactions.push(Transaction{
            sender, 
            receiver,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48;64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self) -> bool {
        self.reward = self.reward / 2f32;
        println!("New reward : {}", self.reward);
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: SystemTime::now(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_transaction = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };
    
        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };
    
        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.curr_transactions);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);
    
        println!("last {:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_transactions: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_transactions {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }

        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        println!("");
        let difficulty = header.difficulty as u64;
        let pb = ProgressBar::new(1024);
        let delta = 8/difficulty;
        let handle = std::thread::spawn(move || {
            for _ in 0..(1024/(delta)) {
                pb.inc(delta);
                thread::sleep(Duration::from_millis(difficulty*10))
            }
            pb.finish_with_message("done");
        });

        let mut m : String = String::from("");
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    }
                    else {
                        m = hash;
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
                
            };
        }

        handle.join().unwrap();
        println!("");
        println!("Block hash: {}", m);
        println!("");
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }

}






