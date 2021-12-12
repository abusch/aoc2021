use std::{iter, str::FromStr};

use anyhow::{Context, Result};
use regex::Captures;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day5.txt")?;
    let data = parse_data(&content)?;

    let mut floor = Floor::new(1000, 1000);
    for line in data.iter().filter(|l| l.is_horizontal_or_vertical()) {
        floor.add_line(line);
    }
    println!("day5 part1 = {}", floor.count_points());

    let mut floor = Floor::new(1000, 1000);
    for line in &data {
        floor.add_line(line);
    }
    println!("day5 part2 = {}", floor.count_points());

    Ok(())
}

#[derive(Debug)]
struct Floor {
    grid: Vec<u64>,
    width: usize,
    height: usize,
}

impl Floor {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: iter::repeat(0).take(width * height).collect(),
            width,
            height,
        }
    }

    fn add_line(&mut self, line: &Line) {
        let Line(Pos(mut x1, mut y1), Pos(mut x2, mut y2)) = *line;

        if x1 == x2 {
            // vertical lines
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                self.set(x1, y);
            }
        } else if y1 == y2 {
            // horizontal lines
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for x in x1..=x2 {
                self.set(x, y1);
            }
        } else {
            // diagonal lines

            if x1 > x2 {
                // Swap the two points so the first point's x is the smallest
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            if y1 <= y2 {
                self.add_line_iter(x1..=x2, y1..=y2);
            } else {
                self.add_line_iter((x1..=x2).rev(), y2..=y1);
            }
        }
    }

    fn add_line_iter(&mut self, xs: impl Iterator<Item = usize>, ys: impl Iterator<Item = usize>) {
        for (x, y) in xs.zip(ys) {
            self.set(x, y);
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.grid[x + y * self.width] += 1;
    }

    fn count_points(&self) -> usize {
        self.grid.iter().copied().filter(|v| *v >= 2).count()
    }
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.grid[x + y * self.width];
                let vstr = format!("{}", v);
                write!(f, "{}", if v == 0 { "." } else { &vstr })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line(Pos, Pos);

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.0 .0 == self.1 .0 || self.0 .1 == self.1 .1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(usize, usize);

fn parse_data(content: &str) -> Result<Vec<Line>> {
    let mut positions = vec![];
    for line in content.lines() {
        let re = regex::Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$")?;
        let cap = re.captures(line).context("Failed to parse line")?;
        let x1 = get_capture(&cap, 1)?;
        let y1 = get_capture(&cap, 2)?;
        let x2 = get_capture(&cap, 3)?;
        let y2 = get_capture(&cap, 4)?;
        positions.push(Line(Pos(x1, y1), Pos(x2, y2)));
    }

    Ok(positions)
}

fn get_capture(cap: &Captures, i: usize) -> Result<usize> {
    cap.get(i)
        .context("no capture group found")
        .and_then(|s| usize::from_str(s.as_str()).context("Failed to parse number"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let data = parse_data(content).unwrap();

        let mut floor = Floor::new(10, 10);
        for line in data.iter().filter(|l| l.is_horizontal_or_vertical()) {
            floor.add_line(line);
        }
        println!("{}", floor);
        assert_eq!(5, floor.count_points());

        let mut floor = Floor::new(10, 10);
        for line in &data {
            floor.add_line(line);
        }
        println!("{}", floor);
        assert_eq!(12, floor.count_points());
    }
}
