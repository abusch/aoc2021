use anyhow::{Context, Result};

struct Crabs(Vec<u64>);

impl Crabs {
    /// Returns the cost of aligning all the crabs to the given horizontal position
    fn align_to(&self, pos: u64) -> u64 {
        self.0
            .iter()
            .copied()
            .map(|p| if p > pos { p - pos } else { pos - p })
            .sum()
    }

    /// Returns the cost of aligning all the crabs to the given horizontal position
    fn align_to2(&self, pos: u64) -> u64 {
        self.0
            .iter()
            .copied()
            .map(|p| {
                let diff = if p > pos { p - pos } else { pos - p };
                if diff > 0 {
                    (diff * (diff + 1)) / 2
                } else {
                    0
                }
            })
            .sum()
    }

    fn cheapest_position(&self) -> u64 {
        let max = self.0.iter().copied().max().unwrap_or(0);
        (0..=max).map(|i| self.align_to(i)).min().unwrap_or(0)
    }

    fn cheapest_position2(&self) -> u64 {
        let max = self.0.iter().copied().max().unwrap_or(0);
        (0..=max).map(|i| self.align_to2(i)).min().unwrap_or(0)
    }
}

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day7.txt")?;
    let positions = content
        .split(',')
        .map(|n| n.trim().parse::<u64>().context("bad input"))
        .collect::<Result<Vec<u64>>>()?;

    let crabs = Crabs(positions);

    let pos = crabs.cheapest_position();
    println!("day7 part1 = {}", pos);

    let pos = crabs.cheapest_position2();
    println!("day7 part2 = {}", pos);


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let crabs = Crabs(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(37, crabs.align_to(2));
    }
}
