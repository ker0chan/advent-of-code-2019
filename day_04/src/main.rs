//123456 => [1,2,3,4,5,6]
fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

//123456 => ['1','2','3','4','5','6']
fn number_to_char_vec(n: u32) -> Vec<char> {
    n.to_string().chars().collect()
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

/* one more important detail:
 * the two adjacent matching digits are not part of a larger group of matching digits.
 */
// We're gonna look for the pattern |xyyz|, with x != y and z != y
fn satisfies_harder_conditions(password: Vec<char>) -> bool {
    let mut has_xyyz = false;
    //Pad the beginning and end of the vector with a non-numeric character so that the search for xyyz works properly on the two extremes
    let mut padded_password = vec!['-'];
    padded_password.extend(&password);
    padded_password.extend(vec!['-']);
    for window in padded_password.windows(4) {
        //First middle digit (y) bigger than the second middle digit: abort
        if window.iter().nth(1).unwrap().to_digit(10).unwrap()
            > window.iter().nth(2).unwrap().to_digit(10).unwrap()
        {
            return false;
        }
        //We need this to be true at least once (exactly two identical adjacent digits)
        has_xyyz = has_xyyz
            || ((window.iter().nth(0).unwrap() != window.iter().nth(1).unwrap())
                && (window.iter().nth(1).unwrap() == window.iter().nth(2).unwrap())
                && (window.iter().nth(2).unwrap() != window.iter().nth(3).unwrap()));
    }
    has_xyyz
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

    // PART 1
    //Count the passwords that meet the criterias!
    let mut candidates = 0;
    //...Across the whole range >:3
    for i in min..max {
        if satisfies_conditions(number_to_vec(i)) {
            candidates += 1
        };
    }

    println!("{}", candidates);

    // PART 2
    //Same thing with slightly different conditions.
    candidates = 0;
    for i in min..max {
        //We want a Vec<char> this time!
        if satisfies_harder_conditions(number_to_char_vec(i)) {
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
    fn part1() {
        assert!(satisfies_conditions(number_to_vec(111111)));
        assert!(!satisfies_conditions(number_to_vec(223450)));
        assert!(!satisfies_conditions(number_to_vec(123789)));
    }
    #[test]
    fn part2() {
        assert!(satisfies_harder_conditions(number_to_char_vec(112233)));
        assert!(!satisfies_harder_conditions(number_to_char_vec(123444)));
        assert!(satisfies_harder_conditions(number_to_char_vec(111122)));
    }
}
