use anyhow::{Context, Result};

#[allow(dead_code)]
pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day14.txt")?;
    let polymer = Polymer::parse(&content)?;

    // 10 steps
    let counters = polymer.count(10);
    println!("day14 part1 = {}", counters.max() - counters.min());

    // 40 steps
    // let counters = polymer.count(30);
    // println!("day14 part2 = {}", counters.max() - counters.min());

    Ok(())
}

#[derive(Debug)]
struct Rules([u8; 26 * 26]);
impl Rules {
    fn new() -> Self {
        Self([0u8; 26 * 26])
    }

    fn set(&mut self, p1: u8, p2: u8, c: u8) {
        let x = (p1 - b'A') as usize;
        let y = (p2 - b'A') as usize;
        self.0[x + y * 26] = c;
    }

    #[inline(always)]
    fn get(&self, p1: u8, p2: u8) -> u8 {
        let x = (p1 - b'A') as usize;
        let y = (p2 - b'A') as usize;
        self.0[x + y * 26]
    }
}

struct Counters([u64; 26]);
impl Counters {
    fn new() -> Self {
        Self([0u64; 26])
    }

    #[inline(always)]
    fn count(&mut self, c: u8) {
        self.0[(c - b'A') as usize] += 1;
    }

    fn get_count(&self, c: u8) -> u64 {
        self.0[(c - b'A') as usize]
    }

    fn min(&self) -> u64 {
        self.0.iter().filter(|&c| *c != 0).min().copied().unwrap()
    }

    fn max(&self) -> u64 {
        self.0.iter().max().copied().unwrap()
    }
}

struct Polymer {
    template: Vec<u8>,
    // rules: HashMap<(u8, u8), u8>,
    rules: Rules,
}

impl Polymer {
    pub fn parse(content: &str) -> Result<Self> {
        let mut lines = content.lines();

        let mut rules = Rules::new();
        let template = lines.next().context("missing template line input")?;
        lines.next().context("missing empty line")?;
        for line in lines {
            let (pair, insert) = line.trim().split_once(" -> ").context("invalid rule")?;
            let (p1, p2) = (pair.as_bytes()[0], pair.as_bytes()[1]);
            let replacement = insert.as_bytes()[0];

            rules.set(p1, p2, replacement);
            // rules.insert((p1, p2), replacement);
        }

        Ok(Self {
            template: template.as_bytes().to_vec(),
            rules,
        })
    }

    fn expand_and_count(&self, counters: &mut Counters, first: u8, second: u8, steps: usize) {
        if steps == 0 {
            counters.count(first);
            return;
        }

        // let new_char = self.rules.get(&(first, second)).copied().unwrap();
        // let mut stack = Vec::with_capacity(steps);

        let new_char = self.rules.get(first, second);
        self.expand_and_count(counters, first, new_char, steps - 1);
        self.expand_and_count(counters, new_char, second, steps - 1);
    }

    fn count(&self, steps: usize) -> Counters {
        let mut counters = Counters::new();

        self.template.windows(2).for_each(|w| {
            println!(".");
            self.expand_and_count(&mut counters, w[0], w[1], steps);
        });
        counters.count(self.template.last().copied().unwrap());

        counters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_and_count() {
        let content = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let polymer = Polymer::parse(content).unwrap();

        let counters = polymer.count(10);
        assert_eq!(1749, counters.get_count(b'B'));
        assert_eq!(298, counters.get_count(b'C'));
        assert_eq!(161, counters.get_count(b'H'));
        assert_eq!(865, counters.get_count(b'N'));
    }
}
