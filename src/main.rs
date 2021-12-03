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

#[derive(Debug)]
struct BitCounter<const N: usize> {
    bits: [u64; N],
    n: u64,
}

impl<const N: usize> BitCounter<N> {
    pub fn new() -> Self {
        Self {
            bits: [0u64; N],
            n: 0,
        }
    }

    fn from_list(numbers: &[&str]) -> Self {
        let mut counter = BitCounter::new();
        numbers.iter().for_each(|n| counter.count_bits(n));
        counter
    }

    fn count_bits(&mut self, bits: &str) {
        bits.bytes().enumerate().for_each(|(i, n)| {
            if n == b'1' {
                self.bits[i] += 1
            }
        });
        self.n += 1;
    }

    fn gamma(&self) -> u64 {
        u64::from_str_radix(&self.most_common(), 2).unwrap()
    }

    fn epsilon(&self) -> u64 {
        u64::from_str_radix(&self.least_common(), 2).unwrap()
    }

    fn most_common(&self) -> String {
        let mut res = String::new();
        for i in 0..N {
            if 2 * self.bits[i] >= self.n {
                res.push('1');
            } else {
                res.push('0');
            }
        }
        res
    }

    fn least_common(&self) -> String {
        let mut res = String::new();
        for i in 0..N {
            if 2 * self.bits[i] < self.n {
                res.push('1');
            } else {
                res.push('0');
            }
        }
        res
    }
}

fn filter_nums<F: Fn(&BitCounter<N>) -> String, const N: usize>(nums: Vec<&str>, f: F) -> u64 {
    let mut remaining = nums;
    for i in 0..N {
        let count = BitCounter::<N>::from_list(&remaining);
        let bits = f(&count);
        // dbg!(&remaining, bits);
        remaining = remaining
            .into_iter()
            .filter(|&n| n.as_bytes()[i] == bits.as_bytes()[i])
            .collect::<Vec<_>>();
        if remaining.len() == 1 {
            break;
        }
    }
    let o2_rating = remaining.first().unwrap();
    u64::from_str_radix(&o2_rating, 2).unwrap()
}

fn day3() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day3.txt")?;
    let lines = data.lines().collect::<Vec<_>>();

    let count = BitCounter::<12>::from_list(&lines);

    println!("Counted {} lines, bit counts = {:?}", count.n, count.bits);
    println!(
        "gamma = {}, epsilon = {}",
        count.most_common(),
        count.least_common()
    );
    let res = count.gamma() * count.epsilon();
    println!(
        "day3 part1 = {} * {} = {}",
        count.gamma(),
        count.epsilon(),
        res
    );

    let o2_rating = filter_nums(lines.clone(), BitCounter::<12>::most_common);
    let co2_rating = filter_nums(lines.clone(), BitCounter::<12>::least_common);

    println!("day3 part2 = {}", o2_rating * co2_rating);

    Ok(())
}

fn main() -> Result<()> {
    day1()?;
    day2()?;
    day3()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_counter() {
        let data = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

        let lines = data.lines().collect::<Vec<_>>();
        let count = BitCounter::<5>::from_list(&lines);
        println!("{}, {}", count.most_common(), count.least_common());
        println!("{}, {}", count.gamma(), count.epsilon());
        assert_eq!(198, count.gamma() * count.epsilon());

        let o2_rating = filter_nums(lines.clone(), BitCounter::<5>::most_common);
        assert_eq!(23, o2_rating);
        let co2_rating = filter_nums(lines.clone(), BitCounter::<5>::least_common);
        assert_eq!(10, co2_rating);
        assert_eq!(230, o2_rating * co2_rating);
    }
}
