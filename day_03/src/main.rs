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

//Eq and Hash are needed for HashSet (along with PartialEq, lower down)
#[derive(Copy, Clone, Eq, Hash)]
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
//Ordering by Manhattan distance with .sort() requires Ord, PartialOrd and PartialEq
use std::cmp::Ordering;
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhattan().cmp(&other.manhattan())
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.manhattan() == other.manhattan()
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
    //Store all the points in a HashSet (it'll tell us if we insert a point that already exists)
    let mut points = HashSet::<Point>::new();
    //Store the intersections, we'll sort them later
    let mut intersections = Vec::new();
    //Where the wires start from
    let origin = Point { x: 0, y: 0 };
    include_str!("input.txt") //Read the input
        .lines()
        //Each line in the file is a "wire"
        .for_each(|wire| {
            let mut current_position = origin;
            //Each wire has a bunch of segments separated by commas
            String::from(wire).split(",").for_each(|segment| {
                //Parse the direction from the first character.
                //This lets us create unit increments in the four directions.
                let direction = Dir::from_char(segment.chars().nth(0).unwrap());
                //The rest of the segment data is its length: parse it, and for every step,
                for _ in 0..segment[1..].parse().unwrap() {
                    //Move to the next point on that segment
                    current_position += direction.to_point();
                    //Try to add the point to the HashSet
                    if !points.insert(current_position) {
                        //Insert returns false on existing values: we already got this point, it's an intersection!
                        intersections.push(current_position);
                    }
                }
            });
        });
    //Sort the intersections by Manhattan distance
    intersections.sort();
    //The first one is the lowest one, that's the answer to Part 1 :3
    println!("{}", intersections[0].manhattan());
}
