//123456 => [1,2,3,4,5,6]
fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

/*
* It is a six-digit number.
* The value is within the range given in your puzzle input.
* Two adjacent digits are the same (like 22 in 122345).
* Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
*/
fn satisfies_conditions(password: Vec<u32>) -> bool {
    let mut has_double = false;
    //For each pair of consecutive digits in the password,
    for window in password.windows(2) {
        //First digit bigger than second digit: abort
        if window.iter().nth(0).unwrap() > window.iter().nth(1).unwrap() {
            return false;
        }
        //We need this to be true at least once (two identical adjacent digits)
        has_double = has_double || (window.iter().nth(0).unwrap() == window.iter().nth(1).unwrap());
    }
    has_double
}

fn main() {
    //Read the input
    let bounds: Vec<u32> = include_str!("input.txt")
        .split("-")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    assert_eq!(bounds.len(), 2);
    //Not really necessary but :shrug:
    let (min, max) = (bounds[0], bounds[1]);

    //Count the passwords that meet the criterias!
    let mut candidates = 0;
    //...Across the whole range >:3
    for i in min..max {
        if satisfies_conditions(number_to_vec(i)) {
            candidates += 1
        };
    }

    println!("{}", candidates);
}

// TESTS
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_ex1() {
        assert!(satisfies_conditions(number_to_vec(111111)));
    }
    #[test]
    fn part1_ex2() {
        assert!(!satisfies_conditions(number_to_vec(223450)));
    }
    #[test]
    fn part1_ex3() {
        assert!(!satisfies_conditions(number_to_vec(123789)));
    }
}
