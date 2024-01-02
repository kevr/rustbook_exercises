// First exercise on https://doc.rust-lang.org/book/ch03-05-control-flow.html
// "Convert temperatures between Fahrenheit and Celsius"
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved
use std::env;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let num_args = _args.len() - 1;

    if num_args != 2 {
        let prog = _args.get(0).unwrap();
        return println!("usage: {prog} (f|c) value");
    }

    let unit = _args.get(1).unwrap().to_lowercase();
    if ! ["f", "c"].iter().any(|&s| s == unit) {
        return println!("error: invalid unit '{unit}'. valid units: f, c");
    }

    let value_str = _args.get(2).unwrap();
    let value = value_str.parse();
    if ! value.is_ok() {
        return println!("error: invalid floating point value '{value_str}'");
    }
    let value: f64 = value.unwrap();

    let output: f64 = if unit == "c" {
        to_celsius(value)
    } else {
        to_fahrenheit(value)
    };

    println!("{output:.1}°{unit}");
}

fn to_celsius(fh: f64) -> f64 {
    (fh - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(c: f64) -> f64 {
    (c / 5.0 * 9.0) + 32.0
}
