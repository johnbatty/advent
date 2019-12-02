use failure::Error;
use std::cmp;
use std::fs;

fn fuel_for_mass(mass: i64) -> i64 {
    if mass == 0 {
        0
    } else {
        let fuel = cmp::max((mass / 3) - 2, 0);
        fuel + fuel_for_mass(fuel)
    }
}

fn parse_mass(mass: &str) -> i64 {
    mass.parse()
        .unwrap_or_else(|e| panic!("Failed to parse mass {}: {}", &mass, &e))
}

fn main() -> Result<(), Error> {
    let total_fuel: i64 = fs::read_to_string("data.txt")?
        .lines()
        .map(parse_mass)
        .map(fuel_for_mass)
        .sum();

    println!("total_fuel: {}", total_fuel);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert!(fuel_for_mass(14) == 2);
        assert!(fuel_for_mass(1969) == 966);
        assert!(fuel_for_mass(100756) == 50346);
    }
}
