
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let start_range: u32 = query("What is the start range?", 1);
    let end_range: u32 = query("What is the end range?", 100);

    let max_guesses = 10;
    let mut guesses_remaining = max_guesses;

    let secret_number = generate_random(start_range, end_range);
    println!("Thinking of a number between {} and {}... done.", start_range, end_range);

    loop {
        println!("Please input your guess. {} remaining.", guesses_remaining);

        let guess: u32 = match read_line().trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);
        guesses_remaining -= 1;

        if guesses_remaining == 0 {
            println!("You lose! The correct answer was {}.", secret_number);
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}

fn query<T>(question: &'static str, default_value: T) -> T where T: std::str::FromStr {
    println!("{}", question);

    return match read_line().trim().parse::<T>() {
        Ok(num) => num,
        Err(_) => default_value
    };
}

fn generate_random(min: u32, max: u32) -> u32 {
    return rand::thread_rng().gen_range(min, max + 1);
}

fn read_line() -> String {
    let mut value = String::new();
    io::stdin().read_line(&mut value)
        .expect("Failed to read line");

    return value;
}