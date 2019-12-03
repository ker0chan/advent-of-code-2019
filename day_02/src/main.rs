//Defines what Intcode instructions look like~
enum IntcodeInstruction {
    Add { lhs: u32, rhs: u32, dest: u32 },
    Mul { lhs: u32, rhs: u32, dest: u32 },
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
            99 => IntcodeInstruction::Halt,
            _ => {
                panic!("Attempting to create an invalid instruction type!");
            }
        }
    }
}

//Executing an instruction means modifying the intcode program, so keep a mutable reference to it!
fn execute_at(index: &mut usize, intcode: &mut [u32]) {
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
    }
}

//Repeatedly execute the instruction at the current index.
fn execute(mut intcode: &mut [u32]) {
    let mut index: usize = 0;

    //Don't run further than expected, and stop on Halt
    while index < intcode.len() {
        //Index will change as soon as we create a new IntcodeInstruction.
        execute_at(&mut index, &mut intcode)
    }
}

fn main() {
    // PART 1
    // INPUT
    let mut intcode = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 2, 23, 6, 27,
        2, 6, 27, 31, 2, 13, 31, 35, 1, 10, 35, 39, 2, 39, 13, 43, 1, 43, 13, 47, 1, 6, 47, 51, 1,
        10, 51, 55, 2, 55, 6, 59, 1, 5, 59, 63, 2, 9, 63, 67, 1, 6, 67, 71, 2, 9, 71, 75, 1, 6, 75,
        79, 2, 79, 13, 83, 1, 83, 10, 87, 1, 13, 87, 91, 1, 91, 10, 95, 2, 9, 95, 99, 1, 5, 99,
        103, 2, 10, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
    ];
    //Initialize the thing
    intcode[1] = 12;
    intcode[2] = 2;
    //Execute the thing
    execute(&mut intcode);
    //Now read the thing >:3
    println!("{}", intcode[0]);
}

// TESTS
#[cfg(test)]
mod tests {
    #[test]
    fn part1_ex1() {
        let mut intcode = [1, 0, 0, 0, 99];
        crate::execute(&mut intcode);
        //Oof.
        // - Get an iterator over that array,
        // - map() each u32 to a String,
        // - collect the values into a Vec (it cannot be a slice, since those are borrows!)
        // - finally, join the pieces together.
        assert_eq!(
            intcode
                .into_iter()
                .map(|i: &u32| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            "2,0,0,0,99"
        );
    }

    #[test]
    fn part1_ex2() {
        let mut intcode = [2, 3, 0, 3, 99];
        crate::execute(&mut intcode);
        assert_eq!(
            intcode
                .into_iter()
                .map(|i: &u32| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            "2,3,0,6,99"
        );
    }

    #[test]
    fn part1_ex3() {
        let mut intcode = [2, 4, 4, 5, 99, 0];
        crate::execute(&mut intcode);
        assert_eq!(
            intcode
                .into_iter()
                .map(|i: &u32| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            "2,4,4,5,99,9801"
        );
    }

    #[test]
    fn part1_ex4() {
        let mut intcode = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        crate::execute(&mut intcode);
        assert_eq!(
            intcode
                .into_iter()
                .map(|i: &u32| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
