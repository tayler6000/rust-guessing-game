use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

const MIN: u32 = 1;
const MAX: u32 = 100;

enum Choice {
    PlayAgain,
    Quit,
}

struct Game {
    secret_number: u32,
    multiplier: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            secret_number: 0,
            multiplier: 1,
        }
    }

    fn get_max(&self) -> u32 {
        MAX * self.multiplier
    }

    fn handle_win(&mut self) -> Choice {
        println!("{}", "You Win!".green());
        println!("\nWould you like to play again? [y/N]");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        if buffer.trim().to_lowercase().eq("y") {
            self.multiplier += 1;
            // TODO: Detect overflow errors and congratulate on being the ultimate winner!
            self.secret_number = rand::thread_rng().gen_range(MIN, self.get_max() + 1);
            return Choice::PlayAgain;
        }
        Choice::Quit
    }

    fn play(&mut self) {
        println!("Guess the number!");

        self.secret_number = rand::thread_rng().gen_range(MIN, self.get_max() + 1);

        loop {
            println!("Please input your guess ({}-{}).", MIN, self.get_max());

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
                Ordering::Equal => match self.handle_win() {
                    Choice::Quit => break,
                    _ => continue,
                },
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
