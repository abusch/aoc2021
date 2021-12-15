use std::collections::HashSet;

use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day15.txt")?;

    let costs = content
        .lines()
        .into_iter()
        .flat_map(|line| line.as_bytes().iter().map(|c| c - b'0'))
        .collect::<Vec<_>>();

    let dijsktra = Dijsktra::new(costs.clone(), 100, 1);
    println!("day15 part1 = {}", dijsktra.run());

    let dijsktra = Dijsktra::new(costs, 100, 5);
    println!("day15 part2 = {}", dijsktra.run());

    Ok(())
}

struct Dijsktra {
    costs: Vec<u8>,
    size: usize,
    ntiles: usize,
}

impl Dijsktra {
    fn new(costs: Vec<u8>, size: usize, ntiles: usize) -> Self {
        Self {
            costs,
            size,
            ntiles,
        }
    }

    fn map_size(&self) -> usize {
        self.size * self.ntiles
    }

    fn run(&self) -> u64 {
        let mut distances = vec![vec![u64::MAX; self.map_size()]; self.map_size()];
        distances[0][0] = 0;
        let mut unvisited = HashSet::new();
        for j in 0..self.map_size() {
            for i in 0..self.map_size() {
                unvisited.insert((i, j));
            }
        }

        let mut current_node = (0, 0);
        loop {
            let (i, j) = current_node;
            let d = distances[i][j];
            for (n, m) in self.neighbours(i, j) {
                if unvisited.contains(&(n, m)) {
                    let new_d = d + self.get_cost(n, m) as u64;
                    if distances[n][m] > new_d {
                        distances[n][m] = new_d;
                    }
                }
            }
            unvisited.remove(&(i, j));
            if current_node == (self.map_size() - 1, self.map_size() - 1) {
                break;
            } else if let Some(pos) = unvisited
                .iter()
                .min_by(|p1, p2| distances[p1.0][p1.1].cmp(&distances[p2.0][p2.1]))
            {
                current_node = *pos;
            } else {
                break;
            }
        }
        distances[self.map_size() - 1][self.map_size() - 1]
    }

    fn get_cost(&self, i: usize, j: usize) -> u8 {
        let n = i % self.size;
        let m = j % self.size;
        let tile_x = (i / self.size) as u8;
        let tile_y = (j / self.size) as u8;
        let cost = self.costs[n + m * self.size] + tile_x + tile_y;
        if cost > 9 {
            cost - 9
        } else {
            cost
        }
    }
    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neigh = Vec::with_capacity(4);
        if i > 0 {
            neigh.push((i - 1, j));
        }
        if i < self.map_size() - 1 {
            neigh.push((i + 1, j));
        }
        if j > 0 {
            neigh.push((i, j - 1));
        }
        if j < self.map_size() - 1 {
            neigh.push((i, j + 1));
        }

        neigh
    }
}
