use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & means it is a reference so we don't need to copy the data into memory multiple times
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {  // shadowing the guess variable, used for type conversions
            Ok(num) => num,
            Err(_) => {
                println!("Try again...");
                continue;
            }, // ignores invalid values
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
