// https://doc.rust-lang.org/book/ch06-02-match.html
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved

enum State {
    California,
}

enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter,
    StateQuarter(State)
}

struct Coin {
    coin: CoinType
}

impl Coin {
    fn new(coin: CoinType) -> Self {
        Self { coin }
    }

    fn cents(&self) -> u8 {
        match self.coin {
            CoinType::Penny => 1,
            CoinType::Nickel => 5,
            CoinType::Dime => 10,
            CoinType::Quarter => 25,
            CoinType::StateQuarter(State::California) => 25
        }
    }

    fn state(&self) -> Option<String> {
        match self.coin {
            CoinType::StateQuarter(State::California) => Some(String::from("California")),
            _ => Some(String::from("N/A"))
        }
    }
}

fn main() {
    let coin = Coin::new(CoinType::Penny);
    let cents = coin.cents();
    println!("Penny: {cents}");

    let coin = Coin::new(CoinType::Nickel);
    let cents = coin.cents();
    println!("Nickel: {cents}");

    let coin = Coin::new(CoinType::Dime);
    let cents = coin.cents();
    println!("Dime: {cents}");

    let coin = Coin::new(CoinType::Quarter);
    let cents = coin.cents();
    println!("Quarter: {cents}");
    let state = coin.state().unwrap();
    println!("State: {state}");

    let coin = Coin::new(CoinType::StateQuarter(State::California));
    let cents = coin.cents();
    println!("Quarter: {cents}");
    let state = coin.state().unwrap();
    println!("State: {state}");
}
