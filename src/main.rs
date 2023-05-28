use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

const MIN: u32 = 1;
const MAX: u32 = 100;
const BIG_MAX: u32 = 1_000_000;

enum Choice {
    PlayAgain,
    Quit,
}

struct Game {
    won_before: bool,
    secret_number: u32
}

impl Game {
    fn new() -> Game{
        Game {
            won_before: false,
            secret_number: 0
        }
    }

    fn handle_win(&mut self) -> Choice {
        self.won_before = true;
        println!("{}", "You Win!".green());
        println!("\nWould you like to play again? [y/N]");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        if buffer.trim().to_lowercase().eq("y") {
            self.secret_number = rand::thread_rng().gen_range(MIN, BIG_MAX);
            return Choice::PlayAgain;
        }
        Choice::Quit
    }

    fn play(&mut self) {
        println!("Guess the number!");

        self.secret_number = rand::thread_rng().gen_range(MIN, MAX);
        self.won_before = false;

        loop {
            let max_in_use = match self.won_before {
                true => BIG_MAX,
                false => MAX
            };
            println!("Please input your guess ({}-{}).", MIN, max_in_use);

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

            match guess.cmp(&self.secret_number) {
                Ordering::Less => println!("{}", "Too small!".yellow()),
                Ordering::Greater => println!("{}", "Too big!".yellow()),
                Ordering::Equal => {
                    match self.handle_win() {
                        Choice::Quit => break,
                        _ => continue
                    }
                }
            }
        }
    }
}

fn main(){
    let mut game = Game::new();
    game.play();
}
