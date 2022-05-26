use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("invalid input {}", guess);
                continue;
            },
        }; 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("the secret number: {}", secret_number);
                println!("You are so rusty!!");
                break;
            },
        }
    }
}
