extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    println!("Generated secret number: {}", secret_number);

    loop {
        println!("Please input your guess");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let mut guess = guess.trim().to_string();
        println!("trimmed guess: {}", guess);

        let guess: u32 = match guess.parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
