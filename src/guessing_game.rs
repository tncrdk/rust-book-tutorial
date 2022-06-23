/* Chapter 2 */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guessing game");

    let number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", number);

    println!("Choose a number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please enter a number");
    println!("You guessed: {}", guess);

    match guess.cmp(&number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
