use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

enum Choice {
    PlayAgain,
    Quit,
}

fn handle_win() -> Choice {
    println!("{}", "You Win!".green());
    println!("\nWould you like to play again? [y/N]");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    if buffer.trim().to_lowercase().eq("y") {
        return Choice::PlayAgain;
    }
    Choice::Quit
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().to_lowercase().eq("quit") {
                    println!("{}", "You loose!".purple());
                    break;
                } else {
                    println!("'{}'", guess)
                }
                println!("{}", "Please enter a number!".red());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Equal => {
                match handle_win() {
                    Choice::Quit => break,
                    _ => continue
                }
            }
        }
    }
}
