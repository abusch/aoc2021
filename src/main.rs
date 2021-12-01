use std::{num::ParseIntError, path::Path};

use anyhow::Result;

fn read_numbers<P: AsRef<Path>>(name: P) -> Result<Vec<u64>> {
    let content = std::fs::read_to_string(name)?;
    let nums: Result<Vec<u64>, ParseIntError> = content.lines().map(|line| line.parse()).collect();

    nums.map_err(|e| e.into())
}

fn day1() -> Result<()> {
    fn check_increase(values: &[u64]) -> u64 {
        values
            .windows(2)
            .map(|values| {
                if values[0] < values[1] { 1 } else { 0 }
            })
            .sum()
    }

    let nums = read_numbers("inputs/day1.txt")?;

    let count: u64 = check_increase(&nums);
    println!("day1 part1 = {}", count);

    let three_sums = nums
        .windows(3)
        .map(|v| v.iter().sum())
        .collect::<Vec<u64>>();
    let count = check_increase(&three_sums);
    println!("day1 part2 = {}", count);

    Ok(())
}

fn main() -> Result<()> {
    day1()?;

    Ok(())
}
