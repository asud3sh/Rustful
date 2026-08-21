use colored::Colorize;
use rand::RngExt;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::process;

fn print_banner() {
    println!("{}", "-----------------------------".yellow());
    println!(
        "{}",
        "👾 Guess the number [1-100] !".bold().black().on_yellow()
    );
    println!("{}", "-----------------------------".yellow());
}

fn print_exit_banner() {
    println!("\n{}", "-----------------------------".yellow());
    println!("   {}", "💛 Thanks for playing!".bold().black().on_yellow());
    println!("{}", "-----------------------------".yellow());
}

fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    // Catch EOF (Ctrl+D) or unexpected stdin closures gracefully
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => {
            print_exit_banner();
            process::exit(0);
        }
        _ => {}
    }
    input.trim().to_lowercase()
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    // Set up Ctrl+C Signal Handler
    ctrlc::set_handler(move || {
        print_exit_banner();
        process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");
    print_banner();

    loop {
        let secret_number = rand::rng().random_range(1..=100);
        let mut attempts = 0;

        loop {
            let input = prompt(&format!("{}", "\n>> ".red().underline()));
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
            print_exit_banner();
            break;
        }

        clear_terminal();
        println!("\nRestarting ...\n");
        print_banner();
    }
}
