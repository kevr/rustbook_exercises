// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved
use std::io::{self, Write};

fn print_flush(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}

fn main() {
    println!("Guess the number!");
    print_flush("Guess: ");
    let message = format!("You guessed: {}", get_input());
    print_flush(message.as_str());
}
