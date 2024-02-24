use read_input::prelude::*;
use std::char;

fn main() {
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
}

fn calc_result(nr1: i32, nr2: i32, op: char) -> i32 {
    let mut res: i32 = 0;
    match op {
        '+' => res = nr1 + nr2,
        '-' => res = nr1 - nr2,
        '*' => res = nr1 * nr2,
        '/' => {
            if nr2 > 0 {
                res = nr1 / nr2;
            }
        }
        _ => {
            println!("\nYou entered an invalid operator");
        }
    };
    res
}
