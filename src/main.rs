use read_input::prelude::*;
use std::char;
use std::io;
use std::io::Write;

fn main() {
    let mut new_run = String::from("y");

    while new_run.trim() == "y" {
        header();

        print!("\n\tEnter your first number:\t");
        let nr1: i32 = input().get();

        print!("\n\tEnter your operator(+,-,*,/):\t");
        let op = input().get();

        print!("\n\tEnter your second number:\t");
        let nr2: i32 = input().get();

        if op == '/' || op == '*' || op == '+' || op == '-' {
            if op == '/' && nr2 == 0 {
                print!("\n\tYou cant divide with 0 or lower");
            } else {
                let res = calc_result(nr1, nr2, op);
                print!("\n\tYour result is: {}", res)
            }
        } else {
            print!("\n\tYou Entered an invalid operator");
        }
        print!("\n\n\tYou want to make an new calculation? (y/n):\t");
        io::stdout().flush().expect("Failed to flush");
        new_run = rerun().to_lowercase();
    }
}

fn header() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\tRust Calculator");
    println!("\t===============");
}

fn rerun() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("\n\tFailed to read input");
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
            println!("\n\tYou entered an invalid operator");
        }
    };
    res
}
