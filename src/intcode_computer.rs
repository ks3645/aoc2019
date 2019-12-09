pub fn run_program(program: &mut Vec<i32>) {
    let mut instr_ptr = 0;

    'eval: loop {
        let instr: OpCode = program[instr_ptr].into();

        match instr {
            OpCode::Halt => { break 'eval; },
            OpCode::Add => {
                let operand_ptr_one = program[instr_ptr + 1] as usize;
                let operand_ptr_two = program[instr_ptr + 2] as usize;

                let result_ptr = program[instr_ptr + 3] as usize;

                program[result_ptr] = program[operand_ptr_one] + program[operand_ptr_two];
            },
            OpCode::Mul => {
                let operand_ptr_one = program[instr_ptr + 1] as usize;
                let operand_ptr_two = program[instr_ptr + 2] as usize;

                let result_ptr = program[instr_ptr + 3] as usize;
                
                program[result_ptr] = program[operand_ptr_one] * program[operand_ptr_two];
            },
            OpCode::Err => {
                panic!("1202 program error");
            }
        }

        instr_ptr += OpCode::value_count(instr);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Add,
    Mul,
    Halt,
    Err,
}

impl std::convert::From<i32> for OpCode {
    fn from(num: i32) -> Self {
        match num {
            1 => OpCode::Add,
            2 => OpCode::Mul,
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
            OpCode::Halt => 1, //maybe not technically correct, but seems to follow from definition of value count as opcode count + parameter count
            _ => {
                panic!("Undefined value count");
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut test_prog = "1,0,0,0,99".trim().split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

        run_program(&mut test_prog);
        assert_eq!(test_prog[..], [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_mul() {
        let mut test_prog = "2,4,4,5,99,0".trim().split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

        run_program(&mut test_prog);
        assert_eq!(test_prog[..], [2, 4, 4, 5, 99, 9801]);
    }
  
}