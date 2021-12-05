use std::str::FromStr;

use anyhow::{bail, Context, Result};

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day2.txt")?;
    let lines = data.lines().collect::<Vec<_>>();
    let moves = lines
        .iter()
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
        let m = split.next().context("invalid input")?;
        let n = split.next().context("Invalid input")?.parse::<i64>()?;

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
