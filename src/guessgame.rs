use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to guessing game");

    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your number :");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too large !"),
            Ordering::Equal => {
                println!("Good job ! You found it !");
                break;
            }
        }
    }
}
