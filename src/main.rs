use core::str;
use read_input::prelude::*;
use std::char;
use std::io;

fn main() {
    let mut new_run = String::from("y");

    while new_run.trim() == "y" {
        let mut res = 0;
        let op;

        println!("\nEnter your first number");
        let nr1: i32 = input().get();

        println!("\nEnter your operator(+,-,*,/)");
        op = input().get();

        println!("\nEnter your second number");
        let nr2: i32 = input().get();

        if op == '/' || op == '*' || op == '+' || op == '-' {
            if op == '/' && nr2 == 0 {
                println!("\nYou cant divide with 0 or lower");
            } else {
                res = calc_result(nr1, nr2, op);
                println!("\nYour result is: {}", res)
            }
        } else {
            println!("\nYou Entered an invalid operator");
        }
        println!("\nYou want to make an new calculation? (y/n)");
        new_run = rerun().to_lowercase();
    }
}

fn rerun() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn calc_result(nr1: i32, nr2: i32, op: char) -> i32 {
    let mut res: i32 = 0;
    match op {
        '+' => res = nr1 + nr2,
        '-' => res = nr1 - nr2,
        '*' => res = nr1 * nr2,
        '/' => res = nr1 / nr2,
        _ => {
            println!("\nYou entered an invalid operator");
        }
    };
    res
}
