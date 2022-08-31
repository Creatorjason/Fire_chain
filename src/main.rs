#![allow(non_snake_case, unused, dead_code)]
#[macro_use]
extern crate serde_derive;

use std::process;
use std::io::{self, Write};

mod fire_chain;
fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();
    println!("Input a miner's address:");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr).unwrap();
    println!("Input difficulty:");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty).unwrap();
    // convert string to integer using the parse method
    let difficulty  = difficulty.trim().parse::<u32>().expect("Integer value required");
    println!("Generating new Genesis block");
    let mut chain = fire_chain::BlockChain::create(difficulty, miner_addr.trim().to_string());
    loop{
        println!("Menu");
        println!("(1) New Transaction");
        println!("(2) Mine block");
        println!("(3) Change Difficulty");
        println!("(4) Change Reward");
        println!("(0) Exit");
        print!("Enter your choice");
        io::stdout().flush();
        choice.clear(); // removes all the content inside the string;
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse::<u32>().unwrap(){
            0 =>{
                println!("Exiting!");
                process::exit(0)
            },
            1 =>{
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                println!("Enter receiver address:");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                println!("Enter amount:"); 
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.create_transaction( sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse::<f32>().unwrap());
                match res{
                    true => println!("Transaction was added successfully"),
                    false => println!("Failed to add transaction")
                }
            },
            2 => {
                println!("Generating block");
                let res = chain.make_block();
                match res{
                    true => println!("New block generated"),
                    false => println!("Failed to generate new block")
                }
            },
            3 =>{
                let mut new_diff = String::new();
                println!("Enter new difficulty:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff).unwrap();
                let res = chain.adjust_difficulty(new_diff.trim().parse::<u32>().unwrap());
                match res {
                    true => println!("Difficulty adjusted successfully"),
                    false => println!("Difficulty adjustment failed")
                }


            } ,
            4 =>{
                let mut new_reward = String::new();
                println!("Enter new reward:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward).unwrap();
                let res = chain.adjust_reward(new_reward.trim().parse::<f32>().unwrap());
                match res {
                    true => println!("Reward adjusted successfully"),
                    false => println!("Reward adjustment failed")
                }
            } ,
            _ => {
                println!("Invalid option please retry")
            },
        }
    }



}
