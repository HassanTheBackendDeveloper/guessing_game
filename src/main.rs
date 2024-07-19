use std::{ io, cmp::Ordering };
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("{}", "Guess the number!".yellow());

    let secret_number = rand::thread_rng().gen_range(0..100);

    loop {
        println!("{}", "Please input your guess.".purple());

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid interger number");
                continue;
            }
        };

        println!("{}", format!("You entered {}", guess).yellow());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
