// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// 'Print the lyrics to the Christmas carol "The Twelve Days of Christmas,"
// taking advantage of the repetition in the song.'
//
// Copyright (C) 2024 Kevin Morris
// All Rights Reserved

const NUMERICS: [&'static str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const LINES: [&'static str; 12] = [
    "Twelve drummers drumming,",
    "Eleven pipers piping,",
    "Ten lords a-leaping,",
    "Nine ladies dancing,",
    "Eight maids a-milking,",
    "Seven swans a-swimming,",
    "Six geese-a-laying,",
    "Five golden rings,",
    "Four calling birds,",
    "Three French hens,",
    "Two turtle doves,",
    "a partridge in a pear tree\n"
];

const STEPS: usize = LINES.len();

fn main() {
    for i in 0..STEPS {
        let numeric = NUMERICS[i];
        println!("On the {numeric} day of Christmas,");
        println!("my true love gave to me");

        let num_lines = LINES.len();
        for j in num_lines - i - 1..num_lines {
            let line = LINES[j];
            
            // Prepend "And " to the last line if we have at least one line to print
            if i > 0 && j == num_lines - 1 {
                print!("And ");
            }

            println!("{line}");
        }
    }
}
