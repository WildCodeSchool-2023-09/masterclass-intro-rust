use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    // let myString: &str = "toto";

    // // -128 -> 127
    // let age: i8 = 4;

    // // 0, 255
    // let age2: u8;

    // let myFloat: f32 = 3.14;

    // let myBool: bool = true;

    // let myArray: [i32; 5] = [1, 2, 3, 4, 5];

    // let myVector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // let myTuple: (i32, f64, &str) = (4, 3.14, "toto");

    // println!("{}", myString);
    // println!("{:?}", myArray);
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
