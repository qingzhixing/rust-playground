use rand;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the guessing game!");
    println!("Choose your difficulty level:");
    println!("1. Easy");
    println!("2. Hard");
    println!("3. Impossible");
    print!("Enter your choice:");
    io::stdout().flush().unwrap(); // 调用flush强制刷新标准输出

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let range_max = match choice.trim() {
        "1" => 100,
        "2" => 1000,
        "3" => 10000,
        _ => {
            println!("Invalid choice. Please enter 1, 2, or 3.");
            return;
        }
    };
    let secret_number = rand::random::<u32>() % range_max + 1;

    println!("The secret number is between 1 and {}", range_max);

    loop {
        print!("Please guess the secret number:");
        io::stdout().flush().unwrap(); // 调用flush强制刷新标准输出

        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed the secret number!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
