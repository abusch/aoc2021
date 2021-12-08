use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use bitflags::bitflags;

bitflags! {
    struct Digit: u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const D = 0b00001000;
        const E = 0b00010000;
        const F = 0b00100000;
        const G = 0b01000000;
    }
}

impl Digit {
    fn num_segments(&self) -> u32 {
        self.bits().count_ones()
    }
}
impl FromStr for Digit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digit = Digit::empty();

        for c in s.chars() {
            let segment = match c {
                'a' => Digit::A,
                'b' => Digit::B,
                'c' => Digit::C,
                'd' => Digit::D,
                'e' => Digit::E,
                'f' => Digit::F,
                'g' => Digit::G,
                _ => bail!("bad segment"),
            };
            digit |= segment;
        }
        Ok(digit)
    }
}

struct Entry {
    patterns: [Digit; 10],
    digits: [Digit; 4],
}

impl Entry {
    fn contains_1478(&self) -> usize {
        // Get the patterns that correspond to the digits 1, 4, 7 or 8 (based on the number of
        // segments)
        let digits_1478 = self
            .patterns
            .iter()
            .filter(|d| {
                let n = d.num_segments();
                n == 2 || n == 4 || n == 3 || n == 7
            })
            .collect::<HashSet<_>>();

        self.digits
            .iter()
            .filter(|d| digits_1478.contains(d))
            .count()
    }

    fn display_value(&self) -> Result<u32> {
        let map = self.map_digits()?;

        let digits = self
            .digits
            .iter()
            .map(|d| map.get(d).context("digit was not mapped!"))
            .collect::<Result<Vec<_>>>()?;

        let res = 1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3];

        Ok(res)
    }

    fn map_digits(&self) -> Result<HashMap<Digit, u32>> {
        let mut map = HashMap::new();
        // Get easy digits
        let d1 = self.get_pattern_with_n_segments(2)?;
        let d4 = self.get_pattern_with_n_segments(4)?;
        let d7 = self.get_pattern_with_n_segments(3)?;
        let d8 = self.get_pattern_with_n_segments(7)?;

        // Get the digits with 5 segments
        let d6s = self.get_patterns_with_n_segments(6);
        let mut d0 = Digit::empty();
        let mut d6 = Digit::empty();
        let mut d9 = Digit::empty();
        for d in d6s {
            if d & d1 != d1 {
                // if it does not "contains" the segments of "1", then it has to be 6
                d6 = d;
            } else if d & d4 == d4 {
                // if it "contains" the segments of "4", then it has to be 9
                d9 = d;
            } else {
                // otherwise, it has to be 0
                d0 = d;
            }
        }

        // Get the digits with 5 segments
        let d5s = self.get_patterns_with_n_segments(5);
        let mut d2 = Digit::empty();
        let mut d3 = Digit::empty();
        let mut d5 = Digit::empty();
        for d in d5s {
            if d & d1 == d1 {
                // if it "contains" the segments of "1", then it has to be 3
                d3 = d;
            } else if d | d1 == d9 {
                d5 = d;
            } else {
                d2 = d;
            }
        }

        map.insert(d0, 0);
        map.insert(d1, 1);
        map.insert(d2, 2);
        map.insert(d3, 3);
        map.insert(d4, 4);
        map.insert(d5, 5);
        map.insert(d6, 6);
        map.insert(d7, 7);
        map.insert(d8, 8);
        map.insert(d9, 9);

        // Make sure we've figured out all digits
        if map.keys().any(Digit::is_empty) {
            bail!("We failed to match all digits!")
        }

        Ok(map)
    }

    fn get_patterns_with_n_segments(&self, n: u32) -> Vec<Digit> {
        self.patterns
            .iter()
            .filter(|d| d.num_segments() == n)
            .copied()
            .collect()
    }

    fn get_pattern_with_n_segments(&self, n: u32) -> Result<Digit> {
        self.get_patterns_with_n_segments(n)
            .first()
            .copied()
            .context("no digit with required number of segments!")
    }
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, digits) = s.split_once(" | ").context("bad input")?;
        let pattern_digits = patterns
            .trim()
            .split_whitespace()
            .map(|f| Digit::from_str(f))
            .collect::<Result<Vec<_>>>()?;
        let display_digits = digits
            .trim()
            .split_whitespace()
            .map(|f| Digit::from_str(f))
            .collect::<Result<Vec<_>>>()?;

        let mut pattern_digits_arr = [Digit::empty(); 10];
        pattern_digits_arr.copy_from_slice(&pattern_digits);

        let mut display_digits_arr = [Digit::empty(); 4];
        display_digits_arr.copy_from_slice(&display_digits);

        Ok(Self {
            patterns: pattern_digits_arr,
            digits: display_digits_arr,
        })
    }
}

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day8.txt")?;
    let entries = content
        .lines()
        .map(Entry::from_str)
        .collect::<Result<Vec<_>>>()?;

    let count = entries.iter().map(|e| e.contains_1478()).sum::<usize>();
    println!("day8 part1 = {}", count);

    let values = entries
        .iter()
        .map(Entry::display_value)
        .collect::<Result<Vec<_>>>()?;
    let sum: u64 = values.iter().map(|v| *v as u64).sum();
    println!("day8 part2 = {}", sum);

    Ok(())
}
