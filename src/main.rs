use std::{num::ParseIntError, path::Path, str::FromStr};

use anyhow::{anyhow, bail, Result};

fn read_numbers<P: AsRef<Path>>(name: P) -> Result<Vec<u64>> {
    let content = std::fs::read_to_string(name)?;
    let nums: Result<Vec<u64>, ParseIntError> = content.lines().map(|line| line.parse()).collect();

    nums.map_err(|e| e.into())
}

fn day1() -> Result<()> {
    fn check_increase(values: &[u64]) -> u64 {
        values
            .windows(2)
            .map(|values| if values[0] < values[1] { 1 } else { 0 })
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let m = split.next().ok_or_else(|| anyhow!("invalid input"))?;
        let n = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid input"))?
            .parse::<i64>()?;

        let res = match m {
            "forward" => Move::Forward(n),
            "up" => Move::Up(n),
            "down" => Move::Down(n),
            _ => bail!("Invalid input!"),
        };

        Ok(res)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Pos {
    horiz: i64,
    depth: i64,
    aim: i64,
}

impl Pos {
    pub fn apply_move(&mut self, m: Move) {
        match m {
            Move::Forward(n) => self.horiz += n,
            Move::Up(n) => self.depth -= n,
            Move::Down(n) => self.depth += n,
        }
    }

    pub fn apply_move2(&mut self, m: Move) {
        match m {
            Move::Forward(n) => {
                self.horiz += n;
                self.depth += self.aim * n;
            }
            Move::Up(n) => self.aim -= n,
            Move::Down(n) => self.aim += n,
        }
    }
}

fn day2() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day2.txt")?;
    let moves = data
        .lines()
        .map(|l| l.parse::<Move>())
        .collect::<Result<Vec<Move>>>()?;

    let res = moves.iter().fold(Pos::default(), |mut acc, m| {
        acc.apply_move(*m);
        acc
    });
    println!("day2 part1: {:?}", res.horiz * res.depth);

    let res = moves.iter().fold(Pos::default(), |mut acc, m| {
        acc.apply_move2(*m);
        acc
    });
    println!("day2 part2: {:?}", res.horiz * res.depth);

    Ok(())
}

fn main() -> Result<()> {
    day1()?;
    day2()?;

    Ok(())
}
