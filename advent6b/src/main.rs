use failure::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn orbital_chain(mut chain: Vec<String>, body: &str, map: &HashMap<&str, &str>) -> Vec<String> {
    if !map.contains_key(body) {
        chain
    } else {
        match map.get(body) {
            Some(parent) => {
                chain.push(parent.to_string());
                orbital_chain(chain, parent, map)
            }
            None => panic!(),
        }
    }
}

fn star_map(map_data: &str) -> HashMap<&str, &str> {
    map_data
        .lines()
        .map(|l| l.split(')').collect_tuple().unwrap())
        .map(|(a, b)| (b, a))
        .collect()
}

fn main() -> Result<(), Error> {
    let map_data = fs::read_to_string("data.txt")?;
    let map = star_map(&map_data);

    let you_chain = orbital_chain(vec![], "YOU", &map);
    let san_chain = orbital_chain(vec![], "SAN", &map);

    for body in &you_chain {
        if san_chain.contains(&body) {
            println!("Go via {}", body);
            let you_dist = you_chain.iter().position(|x| x == body).unwrap();
            let san_dist = san_chain.iter().position(|x| x == body).unwrap();
            println!("Distance to {} you:{} san:{}", body, you_dist, san_dist);
            println!("answer: {}", you_dist + san_dist);
            break;
        }
    }

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
