extern crate rand;

use std::io;
use rand::*;
use std::cmp::Ordering;

fn main() {
    let secret_number0 = rand::thread_rng().gen_range(1, 101);
    println!("Guess what number I'm thinking 🤔");
    // println!("The secret number (0) is {}.", secret_number0);
    loop {
        println!("Type your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to readline D::");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please... TYPE A NUMBER!!");
                continue;
            }
        };

        println!("You guessed: {} !", guess);

        match guess.cmp(&secret_number0) {
            Ordering::Less  => println!("Too small! D:"),
            Ordering::Greater   => println!("Too big! >:("),
            Ordering::Equal  => {
                println!("You got in right! :D");
                break;
            }
        }
    }

}
