use std::{num::ParseIntError, path::Path};

use anyhow::Result;

fn read_numbers<P: AsRef<Path>>(name: P) -> Result<Vec<u64>> {
    let content = std::fs::read_to_string(name)?;
    let nums: Result<Vec<u64>, ParseIntError> = content.lines().map(|line| line.parse()).collect();

    nums.map_err(|e| e.into())
}

fn day1() -> Result<()> {
    let nums = read_numbers("inputs/day1.txt")?;

    let count: u64 = nums.windows(2).map(|values| {
        let res = if values[0] < values[1] { 1 } else { 0 };
        println!("{}, {} → {}", values[0], values[1], res);
        res
    }).sum();

    println!("day1: count = {}", count);
    Ok(())
}

fn main() -> Result<()> {
    day1()?;

    Ok(())
}
