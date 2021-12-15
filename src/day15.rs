use std::collections::BinaryHeap;

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

    fn run(&self) -> i64 {
        // Our goal
        let end = (self.map_size() - 1, self.map_size() - 1);
        // Contains distances from the start node to each node
        let mut distances = vec![vec![i64::MAX; self.map_size()]; self.map_size()];
        distances[0][0] = 0;

        // the nodes to process, ordered by distance to the start
        let mut q = BinaryHeap::new();
        // Initialise the priority queue with the start node
        q.push((0, 0, 0));

        while let Some((r, i, j)) = q.pop() {
            // let (i, j) = current_node;
            let d = distances[i][j];
            if d != -r {
                // If the distance has been updated since we put the node in the queue, just ignore
                // it
                continue;
            }
            for (n, m) in self.neighbours(i, j) {
                let new_d = d + self.get_cost(n, m);
                if new_d < distances[n][m] {
                    distances[n][m] = new_d;
                    q.push((-new_d, n, m));
                }
            }
            // Stop when we've reached the end
            if (i, j) == end {
                break;
            }
        }

        // Return the distance from start to the end node
        distances[end.0][end.1]
    }

    fn get_cost(&self, i: usize, j: usize) -> i64 {
        let n = i % self.size;
        let m = j % self.size;
        let tile_x = (i / self.size) as u8;
        let tile_y = (j / self.size) as u8;
        let cost = self.costs[n + m * self.size] + tile_x + tile_y;
        if cost > 9 {
            cost as i64 - 9
        } else {
            cost as i64
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
