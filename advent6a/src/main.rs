use failure::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn num_orbits(body: &str, h: &HashMap<&str, &str>) -> i32 {
    if !h.contains_key(body) {
        0
    } else {
        match h.get(body) {
            Some(parent) => 1 + num_orbits(parent, h),
            None => panic!(),
        }
    }
}

fn total_orbits(map: &str) -> i32 {
    let h: HashMap<&str, &str> = map
        .lines()
        .map(|l| l.split(')').collect_tuple().unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    h.keys().map(|s| num_orbits(s, &h)).sum()
}

fn main() -> Result<(), Error> {
    let map = fs::read_to_string("data.txt")?;
    println!("{}", total_orbits(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbits() {
        let map = "\
                   COM)B\n\
                   B)C\n\
                   C)D\n\
                   D)E\n\
                   E)F\n\
                   B)G\n\
                   G)H\n\
                   D)I\n\
                   E)J\n\
                   J)K\n\
                   K)L";

        assert!(total_orbits(map) == 42);
    }
}
