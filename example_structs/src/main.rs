// https://doc.rust-lang.org/book/ch05-02-example-structs.html
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Rectangle static methods
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

// Rectangle instance methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let result = rect.area();
    println!("The area of the rectangle is {result} square pixels.");
    println!("Using {:#?}", rect);

    let rect2 = Rectangle::new(10, 40);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::new(60, 45);
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
}
