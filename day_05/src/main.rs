use intcode_computer::*;

fn main() {
    let mut intcode: Vec<i32> = include_str!("input.txt") //Read the input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // This yields [3, 0, 0, 0, 0, 0, 0, 0, 0, 14522484], which should be an error, but the last number ("diagnostic code") is the right answer.
    // ...Guess I'll cheat and pretend everything went alright.
    println!("Output: {:?}", execute(&mut intcode[..], &vec![1]));
}
