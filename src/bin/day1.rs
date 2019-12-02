use std::io::{self, BufRead};

use aoc::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut total = 0u64;
    for line in stdin.lines() {
        let line = line?;
        let mass: u64 = line.parse()?;
        total += fuel_for_mass_and_fuel(mass);
    }

    println!("{}", total);
    Ok(())
}

fn fuel_for_mass(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn fuel_for_mass_and_fuel(mass: u64) -> u64 {
    let mut total = 0;
    let mut uncovered = mass;
    while uncovered > 0 {
        uncovered = fuel_for_mass(uncovered);
        total += uncovered;
    }
    total
}

#[test]
fn test_part1() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);
}

#[test]
fn test_part2() {
    assert_eq!(fuel_for_mass_and_fuel(14), 2);
    assert_eq!(fuel_for_mass_and_fuel(1969), 966);
    assert_eq!(fuel_for_mass_and_fuel(100756), 50346);
}
