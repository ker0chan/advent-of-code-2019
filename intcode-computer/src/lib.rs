enum IntcodeInstruction {
    Add { lhs: u32, rhs: u32, dest: u32 },
    Mul { lhs: u32, rhs: u32, dest: u32 },
    In { dest: u32 },
    Out { src: u32 },
    Halt,
}
impl IntcodeInstruction {
    //Instructions are built from a slice (of which 1 element is read, and perhaps 3 more)
    // We also keep track of the index in the program, and hope we'll only need to move forward through those instructions ^u^'
    fn new(params: &[u32], index: &mut usize) -> Self {
        match params[0] {
            1 => {
                *index += 4;
                IntcodeInstruction::Add {
                    lhs: params[1],
                    rhs: params[2],
                    dest: params[3],
                }
            }
            2 => {
                *index += 4;
                IntcodeInstruction::Mul {
                    lhs: params[1],
                    rhs: params[2],
                    dest: params[3],
                }
            }
            3 => {
                *index += 2;
                IntcodeInstruction::In { dest: params[1] }
            }
            4 => {
                *index += 2;
                IntcodeInstruction::Out { src: params[1] }
            }
            99 => IntcodeInstruction::Halt,
            _ => {
                panic!("Attempting to create an invalid instruction type!");
            }
        }
    }
}

//Executing an instruction means modifying the intcode program, so keep a mutable reference to it!
pub fn execute_at(
    index: &mut usize,
    intcode: &mut [u32],
    input: &Option<u32>,
    output: &mut Vec<u32>,
) {
    let instruction = IntcodeInstruction::new(&intcode[*index..], index);
    match instruction {
        IntcodeInstruction::Add { lhs, rhs, dest } => {
            intcode[dest as usize] = intcode[lhs as usize] + intcode[rhs as usize];
        }
        IntcodeInstruction::Mul { lhs, rhs, dest } => {
            intcode[dest as usize] = intcode[lhs as usize] * intcode[rhs as usize];
        }
        IntcodeInstruction::Halt => {
            //That's one way of making sure the program halts ¯\_(ツ)_/¯ (with the appropriate condition in execute)
            *index = intcode.len()
        }
        IntcodeInstruction::In { dest } => {
            intcode[dest as usize] =
                input.expect("Input instruction cannot be executed without an input!");
        }
        IntcodeInstruction::Out { src } => output.push(intcode[src as usize]),
    }
}

//Repeatedly execute the instruction at the current index.
pub fn execute(mut intcode: &mut [u32], input: Option<u32>) -> Vec<u32> {
    let mut index: usize = 0;
    let mut output = vec![];
    //Don't run further than expected, and stop on Halt
    while index < intcode.len() {
        //Index will change as soon as we create a new IntcodeInstruction.
        execute_at(&mut index, &mut intcode, &input, &mut output);
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
        execute(&mut intcode, None);
        assert_eq!(intcode, [2, 0, 0, 0, 99]);

        let mut intcode = [2, 3, 0, 3, 99];
        execute(&mut intcode, None);
        assert_eq!(intcode, [2, 3, 0, 6, 99]);

        let mut intcode = [2, 4, 4, 5, 99, 0];
        execute(&mut intcode, None);
        assert_eq!(intcode, [2, 4, 4, 5, 99, 9801]);

        let mut intcode = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut intcode, None);
        assert_eq!(intcode, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day5() {
        let mut intcode = [3, 0, 4, 0, 99];
        let output = execute(&mut intcode, Some(3));
        assert_eq!(output, vec![3]);
    }
}
