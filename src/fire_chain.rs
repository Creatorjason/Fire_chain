extern crate sha2;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use sha2::{Sha256, Digest};
use std::fmt::Write;


#[derive(Debug, Serialize)]
pub struct Block{
    trx : Vec<Transaction>,
    header : BlockHeader,
    trx_count : u32
}

#[derive(Serialize, Debug)]
pub struct BlockHeader{
    prev_hash : String,
    timestamp: i64,
    nonce : u32,
    difficulty : u32,
    merkle_root: String,
    block_height: u32,
    
}

#[derive(Serialize,Clone, Debug)]
struct Transaction{
    sender: String,
    receiver : String,
    amount : f32
}

pub struct BlockChain{
    blocks : Vec<Block>,
    trx_list: Vec<Transaction>,
    difficulty: u32,
    miner_addr : String,
    reward: f32
}

impl BlockChain{
    pub fn create(difficulty: u32, miner_addr: String) -> BlockChain{
        let mut fire_chain = BlockChain{
            blocks : vec![],
            trx_list : vec![],
            difficulty,
            miner_addr,
            reward : 200.0 
        };
        fire_chain.make_block();
        fire_chain
    }

    pub fn adjust_difficulty(&mut self, difficulty: u32) -> bool{
        self.difficulty = difficulty;
        true
    }
    pub fn adjust_reward(&mut self, reward: f32) -> bool{
        self.reward = reward;
        true
    }

    pub fn prev_block_hash(&self) -> String{
        let block = match self.blocks.last(){
            Some(block) => block,
            // when converted to string "48" represents 0
            None => return String::from_utf8(vec![48; 64]).expect("Unsupported encoding")
        };
        BlockChain::hash(&block.header)
    }
    pub fn create_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool{
        Transaction{
            sender, 
            receiver,
            amount
        };
        true
    }
    pub fn make_block(&mut self) -> bool{
        let header = BlockHeader{
            prev_hash: self.prev_block_hash(),
            timestamp: Utc::now().timestamp_millis(),
            nonce : 0,
            difficulty:self.difficulty,
            merkle_root: String::new()
        };

        let mut block = Block{
            trx: vec![],
            header,
            trx_count: 0
        };

        let reward_tx = Transaction{
            sender : String::from("JASON"),
            receiver: self.miner_addr.clone(),
            amount : self.reward
        };

       



       
        BlockChain::proof_of_work(&mut block.header);
        block.trx.push(reward_tx);
         // pushing in all trx here because it's not distributed, no other miner plus mem pool
        block.trx.append(&mut self.trx_list); // it moves all the elements from the chain's trx into block trx_list
        block.trx_count = block.trx.len() as u32;
        block.header.merkle_root = BlockChain::get_merkle_root(block.trx.clone());
      

        println!("{:#?}",&block);
        self.blocks.push(block);
        true
        
    }
    fn get_merkle_root(tx_list: Vec<Transaction>) -> String{
        let mut tx_hash = vec![];
         for value in tx_list.iter(){
            let hash = BlockChain::hash(value);
            tx_hash.push(hash);
         }
         if tx_hash.len() % 3 == 0{
            let last = tx_hash.last().cloned();
            match last{
                Some(value) => tx_hash.push(value),
                None => ()
            }
         }

         while tx_hash.len() > 1{
            let mut node_hash_1 = tx_hash.remove(0);
            let mut node_hash_2 = tx_hash.remove(0);
            node_hash_1.push_str(&mut node_hash_2);
            let hashed_join = BlockChain::hash(&node_hash_1);
            tx_hash.push(hashed_join);
         }
         tx_hash.pop().expect("No value returned")
    }
    pub fn proof_of_work(header: &mut BlockHeader){
        loop{
        let mut hash_of_header = BlockChain::hash(header);
        let slice_of_hash = &hash_of_header[..header.difficulty as usize];

        match slice_of_hash.parse::<u32>(){
            Ok(value) => {
                // recall that 00000 or another len is still equal to 0
                if value != 0{
                    header.nonce += 1;
                    println!("hash:{}", hash_of_header);
                }
                else{
                    println!("Calculated block hash :{}", hash_of_header);
                    break;
                }
            },
            Err(_) => {
                header.nonce += 1;
                continue;
            }

        }
    }
}
    pub fn hash<T> (item : &T) -> String
    where T : serde::Serialize
    {
    let input = serde_json::to_string(&item).unwrap(); // serialized
    let mut hash = Sha256::default();// creates new sha256 type
    hash.update(input.as_bytes());// .update accepts the (serialized) byte representation of the input, in this case its a string
    let result = hash.finalize(); // a method that returns the result of a hash function
    let vec_res = result.to_vec(); // converts  bytes into a vector of bytes

    BlockChain::hex_to_string(vec_res.as_slice())

    }
    fn hex_to_string(vec_res : &[u8]) -> String{
    let mut string = String::new();
    for byte in vec_res{
        write!(&mut string, "{:x}", byte).unwrap();
    }
    string
    }
}


