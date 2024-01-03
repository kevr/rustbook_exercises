// Inspired from the dice idea on https://doc.rust-lang.org/book/ch06-02-match.html
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved
mod craps;

use craps::game::Game;

fn main() {
    let mut game = Game::new();
    
    let point = game.come_out();
    println!("Come-out Roll: {point}");

    if game.end_of_turn() {
        if game.push() {
            return println!("Tie!");
        } else if game.nopass() {
            return println!("No Pass!");
        } else if game.pass() {
            return println!("Pass!");
        }
    }

    let mut last: u8 = point;
    while ! game.end_of_turn() {
        last = game.point(point);
        println!("{}", last);
    }

    if last != point {
        println!("Ending the roll...");
    } else {
        println!("Pass!");
    }
}
