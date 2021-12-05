use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day3.txt")?;
    let lines = data.lines().collect::<Vec<_>>();

    let count = BitCounter::<12>::from_list(&lines);

    println!("Counted {} lines, bit counts = {:?}", count.n, count.bits);
    println!(
        "gamma = {}, epsilon = {}",
        count.most_common(),
        count.least_common()
    );
    let res = count.gamma()? * count.epsilon()?;
    println!(
        "day3 part1 = {} * {} = {}",
        count.gamma()?,
        count.epsilon()?,
        res
    );

    let o2_rating = filter_nums(lines.clone(), BitCounter::<12>::most_common);
    let co2_rating = filter_nums(lines.clone(), BitCounter::<12>::least_common);

    println!("day3 part2 = {}", o2_rating * co2_rating);

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

    fn gamma(&self) -> Result<u64> {
        u64::from_str_radix(&self.most_common(), 2).context("Failed to parse binary number")
    }

    fn epsilon(&self) -> Result<u64> {
        u64::from_str_radix(&self.least_common(), 2).context("Failed to parse binay number")
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
    u64::from_str_radix(o2_rating, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_counter() -> Result<()> {
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
        println!("{}, {}", count.gamma()?, count.epsilon()?);
        assert_eq!(198, count.gamma()? * count.epsilon()?);

        let o2_rating = filter_nums(lines.clone(), BitCounter::<5>::most_common);
        assert_eq!(23, o2_rating);
        let co2_rating = filter_nums(lines.clone(), BitCounter::<5>::least_common);
        assert_eq!(10, co2_rating);
        assert_eq!(230, o2_rating * co2_rating);
        Ok(())
    }
}
