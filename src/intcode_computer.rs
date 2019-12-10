use std::collections::VecDeque;

pub fn run_program(program: &mut Vec<i32>, mut input: VecDeque<i32>) -> Vec<i32> {
    let mut output = Vec::new();

    let mut instr_ptr = 0;

    'eval: loop {
        let instr: Instruction = program[instr_ptr].into();

        match instr.op {
            OpCode::Halt => {
                break 'eval;
            }
            OpCode::Add => {
                let result_ptr = program[instr_ptr + 3] as usize;

                program[result_ptr] = get_operand(instr_ptr + 1, instr.modes[0], program)
                    + get_operand(instr_ptr + 2, instr.modes[1], program);

                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::Mul => {
                let result_ptr = program[instr_ptr + 3] as usize;

                program[result_ptr] = get_operand(instr_ptr + 1, instr.modes[0], program)
                    * get_operand(instr_ptr + 2, instr.modes[1], program);

                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::Input => {
                let result_ptr = program[instr_ptr + 1] as usize;
                program[result_ptr] = input.pop_front().unwrap();

                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::Output => {
                output.push(get_operand(instr_ptr + 1, instr.modes[0], program));

                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::JumpIfTrue => {
                if get_operand(instr_ptr + 1, instr.modes[0], program) != 0 {
                    instr_ptr = get_operand(instr_ptr + 2, instr.modes[1], program) as usize;
                } else {
                    instr_ptr += OpCode::value_count(instr.op);
                }
            }
            OpCode::JumpIfFalse => {
                if get_operand(instr_ptr + 1, instr.modes[0], program) == 0 {
                    instr_ptr = get_operand(instr_ptr + 2, instr.modes[1], program) as usize;
                } else {
                    instr_ptr += OpCode::value_count(instr.op);
                }
            }
            OpCode::LessThan => {
                let result_ptr = program[instr_ptr + 3] as usize;
                if get_operand(instr_ptr + 1, instr.modes[0], program)
                    < get_operand(instr_ptr + 2, instr.modes[1], program)
                {
                    program[result_ptr] = 1;
                } else {
                    program[result_ptr] = 0;
                }
                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::Equals => {
                let result_ptr = program[instr_ptr + 3] as usize;
                if get_operand(instr_ptr + 1, instr.modes[0], program)
                    == get_operand(instr_ptr + 2, instr.modes[1], program)
                {
                    program[result_ptr] = 1;
                } else {
                    program[result_ptr] = 0;
                }
                instr_ptr += OpCode::value_count(instr.op);
            }
            OpCode::Err => {
                panic!("1202 program error");
            }
        }
    }

    output
}

fn get_operand(ptr: usize, mode: ParameterMode, program: &Vec<i32>) -> i32 {
    match mode {
        ParameterMode::Position => {
            let operand_ptr = program[ptr] as usize;
            program[operand_ptr]
        }
        ParameterMode::Immediate => program[ptr],
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: OpCode,
    modes: Vec<ParameterMode>,
}

impl std::convert::From<i32> for Instruction {
    fn from(num: i32) -> Self {
        let op = (num % 100).into();
        let mut modes = Vec::new();

        let modes_mask = num / 100;
        match op {
            OpCode::Add | OpCode::Mul | OpCode::LessThan | OpCode::Equals => {
                let mode1 = modes_mask % 10;
                let mode2 = (modes_mask / 10) % 10;

                modes.push(mode1.into());
                modes.push(mode2.into());
                modes.push(ParameterMode::Position); //technically not necessary but being explicit won't hurt for now
            }
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                let mode1 = modes_mask % 10;
                let mode2 = (modes_mask / 10) % 10;

                modes.push(mode1.into());
                modes.push(mode2.into());
            }
            OpCode::Output => {
                let mode1 = modes_mask % 10;

                modes.push(mode1.into());
            }
            _ => {}
        }

        Instruction { op, modes }
    }
}

#[derive(Debug, Copy, Clone)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    Err,
}

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

impl std::convert::From<i32> for ParameterMode {
    fn from(num: i32) -> Self {
        match num {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unrecognized parameter mode"),
        }
    }
}

impl std::convert::From<i32> for OpCode {
    fn from(num: i32) -> Self {
        match num {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => OpCode::Err,
        }
    }
}

impl OpCode {
    fn value_count(instr: OpCode) -> usize {
        match instr {
            OpCode::Add => 4,
            OpCode::Mul => 4,
            OpCode::Input => 2,
            OpCode::Output => 2,
            OpCode::JumpIfTrue => 3,
            OpCode::JumpIfFalse => 3,
            OpCode::LessThan => 4,
            OpCode::Equals => 4,
            OpCode::Halt => 1, //maybe not technically correct, but seems to follow from definition of value count as opcode count + parameter count
            _ => {
                panic!("Undefined value count");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut test_prog = "1,0,0,0,99"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        run_program(&mut test_prog, VecDeque::new());
        assert_eq!(test_prog[..], [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_mul() {
        let mut test_prog = "2,4,4,5,99,0"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        run_program(&mut test_prog, VecDeque::new());
        assert_eq!(test_prog[..], [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_input() {
        let mut test_prog = "3,3,99,0"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut input = VecDeque::new();
        input.push_front(12345);

        run_program(&mut test_prog, input);
        assert_eq!(test_prog[..], [3, 3, 99, 12345]);
    }

    #[test]
    fn test_output() {
        let mut test_prog = "4,3,99,12345"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let output = run_program(&mut test_prog, VecDeque::new());
        assert_eq!(test_prog[..], [4, 3, 99, 12345]);
        assert_eq!(output[..], [12345]);
    }

    #[test]
    fn test_immediate_mode() {
        let mut test_prog = "1101, 100, -1, 4, 0"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        run_program(&mut test_prog, VecDeque::new());
        assert_eq!(test_prog[..], [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_equals() {
        let mut test_prog = "3,3,1108,-1,8,3,4,3,99"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut input = VecDeque::new();
        input.push_front(8);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [1]);

        let mut input = VecDeque::new();
        input.push_front(9);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [0]);
    }

    #[test]
    fn test_less_than() {
        let mut test_prog = "3,9,7,9,10,9,4,9,99,-1,8"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut input = VecDeque::new();
        input.push_front(5);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [1]);

        let mut input = VecDeque::new();
        input.push_front(8);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [0]);
    }

    #[test]
    fn test_jump() {
        let mut test_prog = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut input = VecDeque::new();
        input.push_front(5);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [1]);

        let mut input = VecDeque::new();
        input.push_front(0);

        let output = run_program(&mut test_prog, input);
        assert_eq!(output[..], [0]);
    }
}
