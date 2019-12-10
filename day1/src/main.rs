use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn calculate_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn calculate_fuel_with_fuel(mass: i64) -> i64 {
    let fuel = calculate_fuel(mass);

    if fuel <= 0 {
        0
    } else {
        fuel + calculate_fuel_with_fuel(fuel)
    }
}

fn main() -> io::Result<()> {
    let input = BufReader::new(File::open("inputs/day1.txt")?);

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for line in input.lines() {
        let mass = line?.parse().unwrap();

        part1_sum += calculate_fuel(mass);
        part2_sum += calculate_fuel_with_fuel(mass);
    }

    println!("part 1: {}", part1_sum);
    println!("part 2: {}", part2_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_fuel_with_fuel};

    #[test]
    fn fuel_calculation() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100_756), 33583);
    }

    #[test]
    fn fuel_for_fuel() {
        assert_eq!(calculate_fuel_with_fuel(14), 2);
        assert_eq!(calculate_fuel_with_fuel(1969), 966);
        assert_eq!(calculate_fuel_with_fuel(100_756), 50_346);
    }
}
