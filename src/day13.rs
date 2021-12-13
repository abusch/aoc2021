use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use regex::Regex;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day13.txt")?;

    let mut dots = vec![];
    let mut folds = vec![];
    let mut parsing_positions = true;
    for line in content.lines() {
        if line.is_empty() {
            parsing_positions = false;
            continue;
        }

        if parsing_positions {
            dots.push(Pos::from_str(line)?);
        } else {
            // Parse instruction
            folds.push(Fold::from_str(line)?);
        }
    }
    let mut paper = Paper::new(dots, folds);
    paper.fold_once();
    println!("day13 part1 = {}", paper.count_dots());

    paper.fold_all();
    println!("day13 part2 = \n{}", paper);

    Ok(())
}

#[derive(Debug)]
struct Paper {
    positions: HashSet<Pos>,
    folds: VecDeque<Fold>,
}

impl Paper {
    fn new(positions: Vec<Pos>, folds: Vec<Fold>) -> Self {
        let positions = HashSet::from_iter(positions.into_iter());
        let folds = VecDeque::from_iter(folds.into_iter());

        Self { positions, folds }
    }

    fn fold(&mut self, fold: &Fold) {
        let new_set = self
            .positions
            .iter()
            .map(|p| if fold.affects(p) { fold.mirror(p) } else { *p })
            .collect::<HashSet<_>>();
        self.positions = new_set;
    }

    fn fold_once(&mut self) {
        if let Some(fold) = self.folds.pop_front() {
            self.fold(&fold);
        }
    }

    fn fold_all(&mut self) {
        while let Some(fold) = self.folds.pop_front() {
            self.fold(&fold);
        }
    }

    fn count_dots(&self) -> usize {
        self.positions.len()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.positions.iter().map(|p| p.0).max().unwrap();
        let height = self.positions.iter().map(|p| p.1).max().unwrap();
        for y in 0..=height {
            for x in 0..=width {
                if self.positions.contains(&Pos(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Copy, Hash)]
pub struct Pos(u32, u32);

impl FromStr for Pos {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').context("invalid position")?;
        let x = x.parse::<u32>().context("not a valid number!")?;
        let y = y.parse::<u32>().context("not a valid number!")?;
        Ok(Pos(x, y))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    fn affects(&self, p: &Pos) -> bool {
        match self {
            Fold::X(n) => p.0 > *n,
            Fold::Y(n) => p.1 > *n,
        }
    }

    fn mirror(&self, p: &Pos) -> Pos {
        match self {
            Fold::X(n) => Pos(2 * *n - p.0, p.1),
            Fold::Y(n) => Pos(p.0, 2 * *n - p.1),
        }
    }
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"fold along (x|y)=(\d+)")?;
        let captures = pattern
            .captures(s.trim())
            .context("Invalid fold instructions")?;
        let n = captures
            .get(2)
            .context("invalid instruction")
            .and_then(|m| m.as_str().parse::<u32>().context("invalid number"))?;
        match captures.get(1).map(|m| m.as_str()) {
            Some("x") => Ok(Fold::X(n)),
            Some("y") => Ok(Fold::Y(n)),
            _ => bail!("Invalid axis to fold along"),
        }
    }
}
