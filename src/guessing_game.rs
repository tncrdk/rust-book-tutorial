/* Chapter 2 */

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("\nGUESSING GAME!\n");
    let number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", number);
    loop {
        let mut guess = String::new();
        println!("Choose a number");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number\n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
        println!()
    }
}
