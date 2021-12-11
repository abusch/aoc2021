use std::{fmt::Display, usize};

use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day11.txt")?;

    let mut octopuses = Octopuses::parse(&content);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += octopuses.step();
    }
    println!("day11 part1 = {}", flashes);

    let mut octopuses = Octopuses::parse(&content);
    let mut i = 0;
    loop {
        i += 1;
        let flashes = octopuses.step();
        if flashes == 100 {
            break;
        }
    }
    println!("day11 part2 = {}", i);

    Ok(())
}

struct Octopuses {
    grid: [u8; 100],
}

impl Octopuses {
    pub fn new(data: &[u8]) -> Self {
        let mut grid = [0; 100];
        grid.copy_from_slice(data);

        Self { grid }
    }

    pub fn parse(content: &str) -> Self {
        let data = content
            .lines()
            .flat_map(|line| line.bytes().map(|c| c - b'0'))
            .collect::<Vec<u8>>();
        Self::new(&data)
    }

    fn step(&mut self) -> u64 {
        // increase all energy levels by 1
        self.grid.iter_mut().for_each(|c| *c += 1);
        // flash
        let mut has_flashed = true;
        while has_flashed {
            has_flashed = false;
            for idx in 0..100 {
                // flash if octopus energy level is over 9
                if self.grid[idx] > 9 {
                    self.flash(idx);
                    self.grid[idx] = 0;
                    has_flashed = true;
                }
            }
        }
        // Now reset all the octopuses that flashed
        self.grid.iter().copied().filter(|c| *c == 0).count() as u64
    }

    fn flash(&mut self, idx: usize) {
        let x = idx % 10;
        let y = idx / 10;

        // Now increase all neighbours, taking care of edges
        if x > 0 {
            if y > 0 {
                self.inc(x - 1, y - 1);
            }
            self.inc(x - 1, y);
            if y < 9 {
                self.inc(x - 1, y + 1);
            }
        }
        if y > 0 {
            self.inc(x, y - 1);
        }
        if y < 9 {
            self.inc(x, y + 1);
        }
        if x < 9 {
            if y > 0 {
                self.inc(x + 1, y - 1);
            }
            self.inc(x + 1, y);
            if y < 9 {
                self.inc(x + 1, y + 1);
            }
        }
    }

    fn inc(&mut self, x: usize, y: usize) {
        let idx = x + y * 10;
        if self.grid[idx] != 0 {
            self.grid[idx] += 1;
        }
    }
}

impl Display for Octopuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars: Vec<u8> = self.grid.iter().map(|c| *c + b'0').collect();
        chars
            .chunks(10)
            .try_for_each(|s| writeln!(f, "{}", String::from_utf8_lossy(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut octopuses = Octopuses::parse(content);
        println!("step 0: \n{}", octopuses);
        assert_eq!(0, octopuses.step());
        println!("step 1: \n{}", octopuses);
        let flashes = octopuses.step();
        println!("step 2: \n{}", octopuses);
        assert_eq!(35, flashes);
    }
}
