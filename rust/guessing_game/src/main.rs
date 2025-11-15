use std::cmp::*;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to GUESSING GAME");

    let random = rand::rng().random_range(1..=100);

    let mut no_of_guesses = 0;
    println!("Random Number Generated : 'x'. Enter Yours to Plays.");

    loop {
        println!("Enter your guess:");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Unable to get user input.");

        let num: i32 = match num.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Values entered by user is Invalid..!!");
                continue;
            },
        };

        no_of_guesses += 1;
        // Comparing input with random_number:    
        match num.cmp(&random) {
            Ordering::Equal => {
                println!("Congrats!! Your Guess is Correct..!!");
                break;
            }
            Ordering::Greater => {
                println!("OOPS..!! Your Guess is Bit Higher..!!");
                continue;
            }
            Ordering::Less => {
                println!("OOPS..!! Your Guess is Bit Lower..!!");
                continue;
            }
        }
    }
    println!("\n\nYOU WON..!! TOTAL GUESS TAKEN: {} -> SCORE : {}",no_of_guesses,100/no_of_guesses);
}
