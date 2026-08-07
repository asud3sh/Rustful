use std::io::{self, Write};
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    println!("{}", "-----------------------------".yellow());
    println!("{}", "👾 Guess the number [1-100] !".bold().black().on_yellow());
    println!("{}", "-----------------------------".yellow());

    'game_loop: loop {
        // 1. generate a secret random number
        let secret_number = rand::rng().random_range(1..=100);
        let mut attempts = 0;
        loop {
            // flush to print prompt on same line
            print!("{}", "\n>> ".red().underline());
            io::stdout().flush().expect("Failed to flush stdout");

            // 2. Read input into a String
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            // 3. Parse the string into an integer
            let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}","⚠️  Please enter a valid positive number!".red());
                        continue;
                    }
            };

            // 4. verify if the guess is within range
            if !(1..=100).contains(&guess) {
                println!("{}", "⚠️  Your guess must be between 1 and 100.".blue());
                continue;
            }

            attempts += 1;

            // 5. compare(u32 integers) secret number with the guessed
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Try Higher !".red()),
                Ordering::Equal => {
                    // println!("{}", format!("\n🎉 Perfect Guess, You Won in {} attempts!", attempts).green().bold());
                    println!(
                        "\n🎉 {} {} {}!",
                        "Correct Guess, in".green().bold(),
                        attempts.to_string().yellow().bold(),
                        "attempts".green().bold()
                    );
                    break;
                },
                Ordering::Greater => println!("{}", "Try Lower !".red()),
            }
        }

        // Replay prompt
        print!("{}","\n🙃 Play again? (y/n): ".truecolor(255, 128, 0));
        io::stdout().flush().expect("Failed to flush stdout");

        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read line");

        // Handle case sensitivity
        match replay.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                println!("\nRestarting game...\n");
                clear_terminal();
                println!("{}", "-----------------------------".yellow());
                println!("{}", "👾 Guess the number [1-100] !".bold().black().on_yellow());
                println!("{}", "-----------------------------".yellow());
            },
            _ => {
                println!("{}", "-----------------------------".yellow());
                println!("{}", "💛 Thanks for playing!".bold().black().on_yellow());
                println!("{}", "-----------------------------".yellow());
                break 'game_loop;
            }
        }

    }
}
