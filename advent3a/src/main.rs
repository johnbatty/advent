use failure::Error;
use std::collections::HashSet;
use std::fs;
use std::iter::repeat;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

fn lines_to_points(lines: &str) -> HashSet<Coord> {
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
        .scan(Coord { x: 0, y: 0 }, |coord, step| {
            coord.x += step.0;
            coord.y += step.1;
            Some(*coord)
        })
        .collect()
}

fn intersect_wires(layout: &str) -> i32 {
    let wire_points: Vec<HashSet<Coord>> = layout.lines().map(lines_to_points).collect();

    wire_points[0]
        .intersection(&wire_points[1])
        .map(|coord| coord.x.abs() + coord.y.abs())
        .min()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let wire_data = fs::read_to_string("data.txt")?;
    println!("min_distance: {}", intersect_wires(&wire_data));
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
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];

        for (wires, distance) in &test_data {
            assert_eq!(intersect_wires(wires), *distance);
        }
    }
}
