use crate::utils;
use crate::utils::Part;

use crate::intcode_computer;

use std::collections::VecDeque;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 5).unwrap();

    let mut program: Vec<i32> = input
        .trim()
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let mut input = VecDeque::new();
    match part {
        Part::One => {
            input.push_front(1);
        },
        Part::Two => {
            input.push_front(5);
        }
    }
    

    let output = intcode_computer::run_program(&mut program, input);

    println!("Output: {:?}", output);
    *output.last().unwrap()
}
