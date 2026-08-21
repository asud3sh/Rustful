use colored::Colorize;
use rand::RngExt;
use std::cmp::Ordering;
use std::io::{self, Write};

fn print_banner() {
    println!("{}", "-----------------------------".yellow());
    println!(
        "{}",
        "👾 Guess the number [1-100] !".bold().black().on_yellow()
    );
    println!("{}", "-----------------------------".yellow());
}

fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase()
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    print_banner();

    loop {
        let secret_number = rand::rng().random_range(1..=100);
        let mut attempts = 0;

        loop {
            let input = prompt(&format!("{}", "\n>>".red().underline()));
            let Ok(guess) = input.parse::<u32>() else {
                println!("{}", "⚠️  Please enter a valid positive number!".red());
                continue;
            };

            if !(1..=100).contains(&guess) {
                println!("{}", "⚠️  Your guess must be between 1 and 100.".blue());
                continue;
            }

            attempts += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Try Higher !".red()),
                Ordering::Greater => println!("{}", "Try Lower !".red()),
                Ordering::Equal => {
                    println!(
                        "{}",
                        format!("\n🎉 Correct Guess, in {attempts} attempts!")
                            .green()
                            .bold()
                    );
                    break;
                }
            }
        }

        let replay = prompt(&format!(
            "{}",
            "\n🙃 Play again? (y/n): ".truecolor(255, 128, 0)
        ));

        if !matches!(replay.as_str(), "y" | "yes") {
            println!("{}", "-----------------------------".yellow());
            println!("   {}", "💛 Thanks for playing!".bold().black().on_yellow());
            println!("{}", "-----------------------------".yellow());
            break;
        }

        clear_terminal();
        println!("nRestarting ...\n");
        print_banner();
    }
}
