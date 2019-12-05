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

//Eq, PartialEq, and Hash are needed for HashSet
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
                }
            });
            points
        });
    let wire1 = wires.next().expect("There should be wires");
    let wire2 = wires.next().expect("There should be two wires");
    assert!(
        wires.next().is_none(),
        "There should be *exactly* two wires"
    );
    println!(
        "{}",
        wire1
            .intersection(&wire2) //Find all the points that are on both wires
            .min_by_key(|p| p.manhattan()) //.min_by_key returns the point that gives the minimum value for manhattan() (= the point closest to the origin)
            .unwrap()
            .manhattan()
    );
}
