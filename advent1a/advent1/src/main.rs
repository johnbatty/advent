use failure::Error;
use std::cmp::max;
use std::fs;

fn fuel_for_mass(mass: i64) -> i64 {
    max(0, (mass / 3) - 2)
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

