use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

struct Wire(Vec<Segment>);

impl Deref for Wire {
    type Target = Vec<Segment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Wire {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Result<Vec<_>, _> = s
            .split(',')
            .map(|segment_str| segment_str.parse())
            .collect();
        Ok(Wire(segments?))
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Left => "L",
                Direction::Right => "R",
            }
        )
    }
}

#[derive(Debug)]
struct DirectionParseError;

impl Display for DirectionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse direction")?;

        Ok(())
    }
}

impl Error for DirectionParseError {}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(DirectionParseError),
        };

        Ok(d)
    }
}

struct Segment {
    direction: Direction,
    magnitude: u64,
}

impl FromStr for Segment {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = s.split_at(1);

        Ok(Segment {
            direction: direction.parse()?,
            magnitude: magnitude.parse()?,
        })
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.magnitude)
    }

}

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn manhattan_distance(&self, other: Location) -> u64 {
        u64::try_from((self.x - other.x).abs()).unwrap() + u64::try_from((self.y - other.y).abs()).unwrap()
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.x, self.y))
    }
}

fn trace_wire(wire: &Wire) -> Vec<Location> {
    let mut path = Vec::with_capacity(wire.len());

    let mut location = Location { x: 0, y: 0 };

    path.push(location);

    for vector in wire.iter() {
        for _ in 1..=vector.magnitude {
            match vector.direction {
                Direction::Up => location.y += 1,
                Direction::Down => location.y -= 1,
                Direction::Left => location.x -= 1,
                Direction::Right => location.x += 1,
            }

            path.push(location);
        }
    }

    path
}

fn find_closest_intersection(wire1: &Wire, wire2: &Wire) -> u64 {
    let trace1: HashSet<Location> = HashSet::from_iter(trace_wire(wire1));
    let trace2: HashSet<Location> = HashSet::from_iter(trace_wire(wire2));

    trace1
        .intersection(&trace2)
        .filter(|&&location| location != Location::default())
        .map(|location| location.manhattan_distance(Location::default()))
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/day3.txt")?;
    let mut lines = input.lines();


    let wire1 = lines.next().unwrap().parse()?;
    let wire2 = lines.next().unwrap().parse()?;

    let distance = find_closest_intersection(&wire1, &wire2);

    println!("part 1: {}", distance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::find_closest_intersection;

    #[test]
    fn example1() {
        let wire1 = "R8,U5,L5,D3".parse().unwrap();
        let wire2 = "U7,R6,D4,L4".parse().unwrap();

        assert_eq!(find_closest_intersection(&wire1, &wire2), 6);
    }

    #[test]
    fn example2() {
        let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap();
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap();

        assert_eq!(find_closest_intersection(&wire1, &wire2), 159);
    }

    #[test]
    fn example3() {
        let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".parse().unwrap();
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap();

        assert_eq!(find_closest_intersection(&wire1, &wire2), 135);
    }
}
