use std::io;
use rand::prelude::*;

use std::cmp::Ordering;



fn main(){
    println!("Guessing game");
    
    let secret =rand::rng().random_range(1..=100);
    println!("The secret number is: {secret}");

    println!("please input a number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}");

    
    let guess: u32 = guess.trim().parse().expect("Error while parsing");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You win")
    }

}
