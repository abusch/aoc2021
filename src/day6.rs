use anyhow::{Context, Result};

/*
// Initial naive version
struct Fishies(Vec<u64>);

impl Fishies {
    fn new(pop: &[u64]) -> Self {
        Self(Vec::from_iter(pop.iter().copied()))
    }

    fn step(&mut self) {
        let mut new_fishies = vec![];
        for fish in &mut self.0 {
            if *fish == 0 {
                new_fishies.push(8);
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        self.0.extend(&new_fishies);
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}
*/

// Smarter version: holds a histogram of the population instead
struct Fishies2 {
    population: [u64; 9],
}

impl Fishies2 {
    fn new(pop: &[u64]) -> Self {
        let mut population = [0u64; 9];
        for f in pop {
            population[*f as usize] += 1;
        }
        Self { population }
    }

    fn step(&mut self) {
        let n0 = self.population[0];
        for i in 1..9 {
            self.population[i - 1] = self.population[i];
        }
        self.population[8] = n0;
        self.population[6] += n0;
    }

    fn count(&self) -> u64 {
        self.population.iter().copied().sum()
    }
}

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day6.txt")?;
    let nums = data
        .split(',')
        .map(|n| n.trim().parse::<u64>().context("invalid input"))
        .collect::<Result<Vec<u64>>>()?;

    let mut fishies = Fishies2::new(&nums);
    for _ in 0..80 {
        fishies.step();
    }
    println!("day6 part1 = {}", fishies.count());

    let mut fishies = Fishies2::new(&nums);
    for _ in 0..256 {
        fishies.step();
    }
    println!("day6 part2 = {}", fishies.count());

    Ok(())
}
