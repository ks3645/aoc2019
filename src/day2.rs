use crate::utils;
use crate::utils::Part;

use crate::intcode_computer;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 2).unwrap();

    let mut program: Vec<i32> = input.trim().split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

    match part {
        Part::One => {
            program[1] = 12; //noun
            program[2] = 2; //verb
        
            intcode_computer::run_program(&mut program);
        
            program[0]
        },
        Part::Two => {
            let mut noun = 0;
            let mut verb = 0;
            'outer: for i in 0..100 {
                for j in 0..100 {
                    let mut test_program = program.clone();

                    test_program[1] = i;
                    test_program[2] = j;

                    intcode_computer::run_program(&mut test_program);
                    
                    if test_program[0] == 19690720 {
                        noun = i;
                        verb = j;
                        break 'outer;
                    }
                }
            }
            
            100 * noun + verb
        }
    }
}