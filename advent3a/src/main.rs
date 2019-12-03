use failure::Error;
use std::fs;
use std::collections::HashSet;
use std::cmp::min;

fn lines_to_points(lines: &str) -> HashSet<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut set = HashSet::new();
    for step in lines.split(',') {
        let dir = step[..1].to_string();
        let distance = step[1..].parse::<i32>().unwrap();
        for _ in 0..distance {
            match dir.as_str() {
                "L" => x -= 1,
                "R" => x += 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => panic!("Unexpected direction: {}", step),
            }
            set.insert((x, y));
        }
    }
    set
}

fn intersect_wires(layout: &str) -> i32 {
    let wire_points: Vec<HashSet<(i32, i32)>> = layout
        .lines()
        .map(lines_to_points)
        .collect();
    
    let mut min_distance = i32::max_value();
    for (x, y) in wire_points[0].intersection(&wire_points[1]) {
        println!("Intersection x:{} y:{}", x, y);
        let distance = x.abs() + y.abs();
        println!("distance:{}", distance);
        min_distance = min(min_distance, distance);
    }

    min_distance
}

fn main() -> Result<(), Error> {
    let min_distance = intersect_wires(&fs::read_to_string("data.txt")?);
    println!("min_distance: {}", min_distance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wires() {
        let test_data = [
            ("R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
            U62,R66,U55,R34,D71,R55,D58,R83",
            159),
            ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135),
        ];

        for (wires, distance) in &test_data {
            assert_eq!(intersect_wires(wires), *distance);
        }
    }
}
