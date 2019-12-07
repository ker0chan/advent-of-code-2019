enum ParamMode {
    Position,
    Immediate,
}
impl ParamMode {
    /*  ABCDE
     *  1002
     *
     *  DE - two-digit opcode,      02 == opcode 2
     *  C - mode of 1st parameter,  0 == position mode
     *  B - mode of 2nd parameter,  1 == immediate mode
     *  A - mode of 3rd parameter,  0 == position mode,
     *                                   omitted due to being a leading zero
     */
    fn from_instruction_code(instruction_code: i32, offset: u32) -> Self {
        //Remove the opcode, then %10 to get a single digit
        //offset 0 is the third digit from the end, offset 1 is the fourth digit from the end, and so on.
        let flag = ((instruction_code as u32) / (10u32.pow(2 + offset))) % 10;
        match flag {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!(
                "Invalid flag in instruction code {} at offset {}",
                instruction_code, offset
            ),
        }
    }
}
struct Parameter {
    mode: ParamMode,
    value: i32,
}
impl Parameter {
    /* In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
     * Position mode causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory.
     */
    fn actual_value(&self, intcode: &[i32]) -> i32 {
        match self.mode {
            ParamMode::Position => intcode[self.value as usize],
            ParamMode::Immediate => self.value,
        }
    }
}
enum Instruction {
    Add {
        lhs: Parameter,
        rhs: Parameter,
        dest: Parameter,
    },
    Mul {
        lhs: Parameter,
        rhs: Parameter,
        dest: Parameter,
    },
    In {
        dest: Parameter,
    },
    Out {
        src: Parameter,
    },
    Halt,
}
impl Instruction {
    //Instructions are built from a slice (1 instruction and flags, followed by 0-3 parameters, depending on the type of instruction)
    // We also keep track of the index in the program, and hope we'll only need to move forward through those instructions ^u^'
    fn new(params: &[i32], index: &mut usize) -> Self {
        let opcode = params[0] % 100;
        match opcode {
            1 => {
                *index += 4;
                Instruction::Add {
                    lhs: Parameter {
                        value: params[1],
                        mode: ParamMode::from_instruction_code(params[0], 0),
                    },
                    rhs: Parameter {
                        value: params[2],
                        mode: ParamMode::from_instruction_code(params[0], 1),
                    },
                    dest: Parameter {
                        value: params[3],
                        mode: ParamMode::Immediate,
                    },
                }
            }
            2 => {
                *index += 4;
                Instruction::Mul {
                    lhs: Parameter {
                        value: params[1],
                        mode: ParamMode::from_instruction_code(params[0], 0),
                    },
                    rhs: Parameter {
                        value: params[2],
                        mode: ParamMode::from_instruction_code(params[0], 1),
                    },
                    dest: Parameter {
                        value: params[3],
                        mode: ParamMode::Immediate,
                    },
                }
            }
            3 => {
                *index += 2;
                Instruction::In {
                    dest: Parameter {
                        value: params[1],
                        mode: ParamMode::Immediate,
                    },
                }
            }
            4 => {
                *index += 2;
                Instruction::Out {
                    src: Parameter {
                        value: params[1],
                        mode: ParamMode::Immediate,
                    },
                }
            }
            99 => Instruction::Halt,
            _ => {
                panic!(
                    "Attempting to create an invalid instruction type: \"{}\" (from instruction data: {})",
                    opcode, params[0]
                );
            }
        }
    }
}

//Executing an instruction means modifying the intcode program, so keep a mutable reference to it!
pub fn execute_at(
    index: &mut usize,
    intcode: &mut [i32],
    input_iter: &mut dyn Iterator<Item = &i32>,
    output: &mut Vec<i32>,
) {
    //Build an instruction from data at the current position, and move the index
    let instruction = Instruction::new(&intcode[*index..], index);
    match instruction {
        Instruction::Add { lhs, rhs, dest } => {
            intcode[dest.value as usize] = lhs.actual_value(&intcode) + rhs.actual_value(&intcode);
        }
        Instruction::Mul { lhs, rhs, dest } => {
            intcode[dest.value as usize] = lhs.actual_value(&intcode) * rhs.actual_value(&intcode);
        }
        Instruction::Halt => {
            //That's one way of making sure the program halts ¯\_(ツ)_/¯ (with the appropriate condition in execute)
            *index = intcode.len()
        }
        Instruction::In { dest } => {
            intcode[dest.value as usize] = *input_iter
                .next() //Consumes an input
                .expect("Input instruction cannot be executed without an input!");
        }
        Instruction::Out { src } => output.push(intcode[src.value as usize]),
    }
}

//Execute the intcode program, consuming inputs from the input vector, and returning an output vector.
pub fn execute(mut intcode: &mut [i32], input: &Vec<i32>) -> Vec<i32> {
    //Current execution index
    let mut index: usize = 0;
    //Output vector
    let mut output = vec![];
    //Iterator on the input vector, can be mutated on execution
    let mut input_iter = (*input).iter();
    //Stop running at the end of the program (a Halt instruction will move the execution index at the end of the program too)
    while index < intcode.len() {
        //Build and execute a single instruction
        execute_at(&mut index, &mut intcode, &mut input_iter, &mut output);
    }
    output
}

// TESTS
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn day2() {
        let mut intcode = [1, 0, 0, 0, 99];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [2, 0, 0, 0, 99]);

        let mut intcode = [2, 3, 0, 3, 99];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [2, 3, 0, 6, 99]);

        let mut intcode = [2, 4, 4, 5, 99, 0];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [2, 4, 4, 5, 99, 9801]);

        let mut intcode = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day5() {
        let mut intcode = [3, 0, 4, 0, 99];
        let output = execute(&mut intcode, &vec![3]);
        assert_eq!(output, vec![3]);

        let mut intcode = [1101, 100, -1, 4, 0];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [1101, 100, -1, 4, 99]);

        let mut intcode = [1002, 4, 3, 4, 33];
        execute(&mut intcode, &vec![]);
        assert_eq!(intcode, [1002, 4, 3, 4, 99]);

        let mut intcode = [3, 0, 1, 0, 6, 6, 1];
        execute(&mut intcode, &vec![98]);
        assert_eq!(intcode, [98, 0, 1, 0, 6, 6, 99]);
    }
}
