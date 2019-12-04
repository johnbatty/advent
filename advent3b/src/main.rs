use failure::Error;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::iter::repeat;

#[derive(Eq, Clone, Copy)]
struct WirePoint {
    x: i32,
    y: i32,
    s: i32, // Step count
}

// Deliberately ignore the `s` field when calculating hash
impl Hash for WirePoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

// Deliberately ignore the `s` field when testing equality
impl PartialEq for WirePoint {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

fn lines_to_points(lines: &str) -> HashSet<WirePoint> {
    lines
        .split(',')
        .map(|s| s.split_at(1))
        .flat_map(|(dir, len)| {
            repeat(match dir {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Invalid direction: {}", dir),
            })
            .take(len.parse::<usize>().unwrap())
        })
        .scan(WirePoint { x: 0, y: 0, s: 0 }, |coord, step| {
            coord.x += step.0;
            coord.y += step.1;
            coord.s += 1;
            Some(*coord)
        })
        .unique()
        .collect()
}

fn intersect_wires(layout: &str) -> i32 {
    let wire_points: Vec<HashSet<WirePoint>> = layout.lines().map(lines_to_points).collect();

    wire_points[0]
        .iter()
        .filter(|wp| wire_points[1].contains(&wp))
        .map(|wp| wire_points[0].get(&wp).unwrap().s + wire_points[1].get(&wp).unwrap().s)
        .min()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let wire_data = fs::read_to_string("data.txt")?;
    println!("distance: {}", intersect_wires(&wire_data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wires() {
        let test_data = [
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                 U62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ];

        for (wires, distance) in &test_data {
            assert_eq!(intersect_wires(wires), *distance);
        }
    }
}
