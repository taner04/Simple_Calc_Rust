use read_input::prelude::*;
use std::io;
use std::io::Write;

fn main() {
    let mut new_run = String::from("y");
    let op_list: [String; 4] = [
        "+".to_string(),
        "-".to_string(),
        "/".to_string(),
        "*".to_string(),
    ];

    while new_run.trim() == "y" {
        header();

        println!("\n\tEnter your first number: ");
        let nr1: i32 = input().get();

        println!("\n\tEnter your operator(+,-,*,/): ");
        let op = user_input().trim().to_string();

        println!("\n\tEnter your second number: ");
        let nr2: i32 = input().get();

        if op_list.contains(&op.to_string()) {
            if op == "/" && nr2 == 0 {
                println!("\n\tYou cant divide with 0 or lower");
            } else {
                let res = calc_result(nr1, nr2, op.as_str());
                println!("\n\tYour result is: {}", res)
            }
        } else {
            println!("\n\tYou Entered an invalid operator");
        }
        print!("\n\tYou want to make an new calculation? (y/n): ");
        io::stdout().flush().expect("Failed to flush");
        new_run = user_input().to_lowercase();
    }
}

fn header() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\tRust Calculator");
    println!("\t===============");
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("\n\tFailed to read input");
    input
}

fn calc_result(nr1: i32, nr2: i32, op: &str) -> i32 {
    let mut res: i32 = 0;
    match op {
        "+" => res = nr1 + nr2,
        "-" => res = nr1 - nr2,
        "*" => res = nr1 * nr2,
        "/" => res = nr1 / nr2,
        _ => {
            println!("\n\tYou entered an invalid operator");
        }
    };
    res
}
