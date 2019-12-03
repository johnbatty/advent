use failure::Error;
use std::collections::HashMap;
use std::fs;

fn lines_to_points(lines: &str) -> HashMap<(i32, i32), i32> {
    let mut x = 0;
    let mut y = 0;
    let mut step_count = 0;
    let mut steps = HashMap::new();
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
            step_count += 1;
            if !steps.contains_key(&(x, y)) {
                steps.insert((x, y), step_count);
            }
        }
    }
    steps
}

fn intersect_wires(layout: &str) -> i32 {
    let wire_steps: Vec<HashMap<(i32, i32), i32>> = layout.lines().map(lines_to_points).collect();

    wire_steps[0]
        .keys()
        .filter(|(x, y)| wire_steps[1].contains_key(&(*x, *y)))
        .map(|(x, y)| wire_steps[0].get(&(*x, *y)).unwrap() + wire_steps[1].get(&(*x, *y)).unwrap())
        .min()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let distance = intersect_wires(&fs::read_to_string("data.txt")?);
    println!("distance: {}", distance);
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
