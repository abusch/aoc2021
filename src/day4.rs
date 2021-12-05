use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::{Context, Result};

#[derive(Debug, Default)]
struct Bingo {
    numbers: VecDeque<u64>,
    boards: Vec<Board>,
    winners: Vec<(usize, u64)>,
}

impl Bingo {
    fn parse(data: &str) -> Result<Self> {
        let mut lines = data.lines();

        // parse first line of drawn numbers
        let numbers = lines.next().context("Invalid input data")?;
        let numbers = numbers
            .split(',')
            .map(u64::from_str)
            .collect::<Result<VecDeque<_>, _>>()?;

        // skip blank line
        lines.next();

        // Parse boards
        let mut boards = Vec::new();
        let mut cur_grid = Vec::with_capacity(25);
        for line in lines {
            if line.is_empty() {
                boards.push(Board::new(&cur_grid));
                cur_grid.clear();
                continue;
            }
            line.split_ascii_whitespace().for_each(|n| {
                let num = u64::from_str(n).expect("Invalid number");
                cur_grid.push(num);
            });
        }

        // last board if there was no empty line at the end
        if !cur_grid.is_empty() {
            boards.push(Board::new(&cur_grid));
        }

        Ok(Self {
            numbers,
            boards,
            winners: vec![],
        })
    }

    fn play(&mut self) {
        for draw in &self.numbers {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if !board.is_winning() {
                    board.mark_number(*draw);
                    if board.is_winning() {
                        self.winners.push((i, *draw));
                    }
                }
            }
        }
    }

    fn first_winner(&self) -> Option<(&Board, u64)> {
        self.winners.first().map(|(i, n)| (&self.boards[*i], *n))
    }

    fn last_winner(&self) -> Option<(&Board, u64)> {
        self.winners.last().map(|(i, n)| (&self.boards[*i], *n))
    }
}

#[derive(Default)]
struct Board {
    grid: [u64; 25],
    marks: [bool; 25],
}

impl Board {
    fn new(nums: &[u64]) -> Self {
        assert!(nums.len() == 25);
        let mut board = Self::default();
        board.grid.copy_from_slice(nums);

        board
    }

    fn mark_number(&mut self, draw: u64) {
        for (i, n) in self.grid.iter().enumerate() {
            if *n == draw {
                self.marks[i] = true;
            }
        }
    }

    fn is_winning(&self) -> bool {
        // check rows
        for row in 0..5 {
            let mut all_same = true;
            for col in 0..5 {
                all_same &= self.marks[col + 5 * row];
            }
            if all_same {
                return true;
            }
        }

        // check columns
        for col in 0..5 {
            let mut all_same = true;
            for row in 0..5 {
                all_same &= self.marks[col + 5 * row];
            }
            if all_same {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u64 {
        self.grid
            .iter()
            .copied()
            .zip(self.marks.iter().copied())
            .filter(|(_n, m)| !*m)
            .map(|(n, _m)| n)
            .sum()
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                write!(
                    f,
                    "{:2}{} ",
                    self.grid[col + row * 5],
                    if self.marks[col + row * 5] { '*' } else { ' ' }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day4.txt")?;
    let mut bingo = Bingo::parse(&data)?;

    bingo.play();
    let (winner, last_number) = bingo.first_winner().expect("No winning board!");
    println!("day4 part1: {}", winner.score() * last_number);

    let (winner, last_number) = bingo.last_winner().expect("No winning board!");
    println!("day4 part2: {}", winner.score() * last_number);

    Ok(())
}
