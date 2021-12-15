use std::collections::HashSet;

use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day15.txt")?;

    let costs = content
        .lines()
        .into_iter()
        .flat_map(|line| line.as_bytes().iter().map(|c| c - b'0'))
        .collect::<Vec<_>>();

    let mut distances = [[u64::MAX; 100]; 100];
    distances[0][0] = 0;
    let mut unvisited = HashSet::new();
    for j in 0..100 {
        for i in 0..100 {
            unvisited.insert((i, j));
        }
    }

    let mut current_node = (0, 0);
    loop {
        let (i, j) = current_node;
        let d = distances[i][j];
        for (n, m) in neighbours(i, j) {
            if unvisited.contains(&(n, m)) {
                let new_d = d + costs[n + 100 * m] as u64;
                if distances[n][m] > new_d {
                    distances[n][m] = new_d;
                }
            }
        }
        unvisited.remove(&(i, j));
        if current_node == (99, 99) {
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

    println!("day15 part1 = {}", distances[99][99]);

    Ok(())
}

fn neighbours(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neigh = Vec::with_capacity(4);
    if i > 0 {
        neigh.push((i - 1, j));
    }
    if i < 99 {
        neigh.push((i + 1, j));
    }
    if j > 0 {
        neigh.push((i, j - 1));
    }
    if j < 99 {
        neigh.push((i, j + 1));
    }

    neigh
}
