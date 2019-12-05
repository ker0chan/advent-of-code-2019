//Fancier Up Down Left Right
enum Dir {
    North,
    South,
    East,
    West,
}
impl Dir {
    //Directions are built from a char
    fn from_char(c: char) -> Dir {
        match c {
            'D' => Dir::South,
            'U' => Dir::North,
            'L' => Dir::West,
            'R' => Dir::East,
            _ => panic!("Unknown direction {}", c),
        }
    }

    //Returns unit vectors we can add to points to follow that direction
    fn to_point(&self) -> Point {
        match self {
            Dir::South => Point { x: 0, y: -1 },
            Dir::North => Point { x: 0, y: 1 },
            Dir::West => Point { x: -1, y: 0 },
            Dir::East => Point { x: 1, y: 0 },
        }
    }
}

//Eq, PartialEq, and Hash are needed for HashSet/HashMap
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
//Trait required to add points together
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// https://en.wikipedia.org/wiki/Taxicab_geometry
impl Point {
    fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    //Where the wires start from
    let origin = Point { x: 0, y: 0 };
    let mut wires = include_str!("input.txt") //Read the input
        .lines()
        //Each line in the file is a "wire"
        .map(|wire| {
            let mut current_position = origin;
            //Store all the points in a HashSet
            let mut points = HashSet::<Point>::new();
            let mut steps_taken = HashMap::<Point, u32>::new();
            let mut steps = 0;
            //Each wire has a bunch of segments separated by commas
            String::from(wire).split(",").for_each(|segment| {
                //Parse the direction from the first character.
                //This lets us create unit increments in the four directions.
                let direction = Dir::from_char(segment.chars().nth(0).unwrap());
                //The rest of the segment data is its length: parse it, and for every step,
                for _ in 0..segment[1..].parse().unwrap() {
                    //Move to the next point on that segment
                    current_position += direction.to_point();
                    //Try to add the point to the HashSet (self-intersections won't work but we ignore those)
                    points.insert(current_position);
                    //Keep track of the number of steps it took to reach that point
                    steps += 1;
                    //Note that HashMap won't let us overwrite the amount of steps if we reach a point twice. That's useful!
                    steps_taken.insert(current_position, steps);
                }
            });
            (points, steps_taken)
        });
    let (wire1_points, wire1_steps) = wires.next().expect("There should be wires");
    let (wire2_points, wire2_steps) = wires.next().expect("There should be two wires");
    assert!(
        wires.next().is_none(),
        "There should be *exactly* two wires"
    );

    // PART 1
    println!(
        "{}",
        wire1_points
            .intersection(&wire2_points) //Find all the points that are on both wires
            .min_by_key(|p| p.manhattan()) //.min_by_key returns the point that gives the minimum value for manhattan() (= the point closest to the origin)
            .unwrap()
            .manhattan() //The lowest manhattant distance is the answer to part 1 :3
    );

    // PART 2
    println!(
        "{}",
        wire1_points
            .intersection(&wire2_points) //Find all the points that are on both wires
            .map(|p| wire1_steps[p] + wire2_steps[p]) //Figure out how many steps it took to get there
            .min() //The amount of steps it took to reach the earliest intersection is the answer to part 2 :3
            .unwrap()
    );
}
