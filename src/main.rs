use rand::Rng;
use std::{cmp::Ordering, io,};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The limit bound of random number
   #[arg(short, long, default_value_t = 100)]
   limit: u32,
}

fn main() {
    let args = Args::parse();

    println!("Guess the number gameeeeeee!");

    let secret_number = rand::thread_rng().gen_range(1..=args.limit);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline :/");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! try again:"),
            Ordering::Greater => println!("Too big! try again:"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
