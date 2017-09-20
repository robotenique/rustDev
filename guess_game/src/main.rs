extern crate rand;

use std::io;
use rand::*;
use std::cmp::Ordering;
use rand::distributions::IndependentSample;

fn main() {
    let secret_number0 = rand::thread_rng().gen_range(1, 101);
    let secret_number1 = rand::distributions::normal::Normal::new(50.0, 50.0)
                        .ind_sample(&mut rand::thread_rng());
    println!("Guess what number I'm thinking 🤔");
    println!("The secret number (0) is {}.", secret_number0);
    //println!("The ULTRA secret number (1) is {}.", secret_number1 as i64);
    println!("Type your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to readline D::");

    let guess: u32 = guess.trim().parse()
                .expect("Please... TYPE A NUMBER!!");;

    println!("You guessed: {} !", guess);

    match guess.cmp(&secret_number0) {
        Ordering::Less  => println!("Too small! D:"),
        Ordering::Greater   => println!("Too big! >:("),
        Ordering::Equal  => println!("You got in right! :D")
    }


}
