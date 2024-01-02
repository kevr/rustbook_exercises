// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// "Generate the nth Fibonacci number."
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved
use std::env;
use std::collections::HashMap;

fn main() {
    // Gather command-line arguments
    let _args: Vec<String> = env::args().collect();
    if _args.len() != 2 {
        let exec: &String = _args.get(0).unwrap();
        return println!("usage: {exec} n");
    }

    // Parse n out of the command-line
    let n_str = _args.get(1).unwrap();
    let n: Result<u128, _> = n_str.parse();
    if ! n.is_ok() {
        return println!("error: invalid value '{n_str}', n must be an integer");
    }
    let n: u128 = n.unwrap();

    // Find the nth fibonacci number
    let answer = fibonacci(n);
    println!("{answer}");
}

fn fibonacci(n: u128) -> u128 {
    let mut memo: HashMap<u128, u128> = HashMap::new();
    fibo(n, &mut memo)
}

fn fibo(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n < 2 {
        return n;
    }

    if memo.contains_key(&n) {
        return memo.get(&n).unwrap().clone();
    }

    let rhs = fibo(n - 2, memo);
    let lhs = fibo(n - 1, memo);
    let ans = lhs + rhs;
    memo.insert(n, ans);
    return ans;
}
