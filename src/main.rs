#[macro_use]
extern crate serde_derive;
extern crate indicatif;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_address: String = String::new();
    let mut difficulty: String = String::new();
    let mut choice: String = String::new();

    get_input("Input a miner address: ", &mut miner_address);
    get_input("Difficulty: ", &mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need.");
    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop {
        println!("MENU");
        println!("1. New Transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change Reward");
        println!("0. EXIT");
        println!("Enter your choice: ");

        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");


        //Menu
        match choice.trim().parse().unwrap() {
            0 => {
                println!("EXITING");
                process::exit(0);
            }, 
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                get_input("enter sender andress: ", &mut sender);
                get_input("enter receiver andress: ", &mut receiver);
                get_input("enter amount: ", &mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), 
                                                receiver.trim().to_string(),
                                                amount.trim().parse().unwrap());
                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generate successfully"),
                    false => println!("Block generation failed"),
                }
            },

            3 => {
                let mut new_diff = String::new();
                get_input("enter new difficulty: ", &mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Update Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            },
            
            4 => {
                let res = chain.update_reward();
                match res {
                    true => println!("Update reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => println!("Invalid option please retry"),

        }
    }
}



fn get_input(ask_message : &str, s : &mut String) {
    println!("{}", ask_message);
    io::stdout().flush();
    io::stdin().read_line(s);
}

