use intcode_computer::*;

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
    execute(&mut intcode, &vec![]);
    //Now read the thing >:3
    println!("{}", intcode[0]);

    //PART 2
    //Is there any point in NOT trying every combination here?
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = [
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 2, 23,
                6, 27, 2, 6, 27, 31, 2, 13, 31, 35, 1, 10, 35, 39, 2, 39, 13, 43, 1, 43, 13, 47, 1,
                6, 47, 51, 1, 10, 51, 55, 2, 55, 6, 59, 1, 5, 59, 63, 2, 9, 63, 67, 1, 6, 67, 71,
                2, 9, 71, 75, 1, 6, 75, 79, 2, 79, 13, 83, 1, 83, 10, 87, 1, 13, 87, 91, 1, 91, 10,
                95, 2, 9, 95, 99, 1, 5, 99, 103, 2, 10, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99,
                2, 14, 0, 0,
            ];
            intcode[1] = noun;
            intcode[2] = verb;
            execute(&mut intcode, &vec![]);
            if intcode[0] == 19690720 {
                println!(
                    "verb: {}, noun:{}, result:{}",
                    verb,
                    noun,
                    100 * noun + verb
                );
                //Let's not be entirely horrible and stop when we reach the solution, ok
                break 'outer;
            }
        }
    }
}
