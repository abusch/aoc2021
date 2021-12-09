use std::collections::HashSet;

use anyhow::{Context, Result, bail};

pub struct Heightmap {
    grid: Vec<u8>,
    height: usize,
    width: usize,
}

impl Heightmap {
    pub fn new(grid: Vec<u8>, height: usize, width: usize) -> Self {
        Self {
            grid,
            height,
            width,
        }
    }

    pub fn from_str(content: &str) -> Result<Self> {
        let width = content
            .lines()
            .next()
            .context("invalid input")?
            .trim()
            .len();
        let height = content.lines().count();

        let to_digit = |c: char| match c {
            '0' => Ok(0),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            _ => bail!("not a digit!"),
        };
        let data = content
            .lines()
            .flat_map(|line| line.chars().map(to_digit))
            .collect::<Result<Vec<u8>>>()?;

        Ok(Heightmap::new(data, height, width))
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[x + y * self.width]
    }

    fn get_neighbours_idx(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push(self.pos2idx(x - 1, y));
        }
        if y > 0 {
            neighbours.push(self.pos2idx(x, y - 1));
        }
        if x < self.width - 1 {
            neighbours.push(self.pos2idx(x + 1, y));
        }
        if y < self.height - 1 {
            neighbours.push(self.pos2idx(x, y + 1));
        }

        neighbours
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<u8> {
        self.get_neighbours_idx(x, y)
            .into_iter()
            .map(|idx| self.grid[idx])
            .collect()
    }

    fn get_low_points_idx(&self) -> Vec<usize> {
        let mut low_points = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbours = self.get_neighbours(x, y);
                let h = self.get(x, y);
                if neighbours.iter().all(|v| h < *v) {
                    low_points.push(self.pos2idx(x, y));
                }
            }
        }
        low_points
    }

    fn get_low_points(&self) -> Vec<u8> {
        self.get_low_points_idx()
            .into_iter()
            .map(|idx| self.grid[idx])
            .collect()
    }

    fn get_risk_level_sum(&self) -> u64 {
        self.get_low_points()
            .into_iter()
            .map(|v| v as u64 + 1)
            .sum::<u64>()
    }

    fn get_3_largest_basins(&self) -> Vec<usize> {
        let mut basins = self.get_basin_sizes();
        basins.sort_unstable();
        basins.into_iter().rev().take(3).collect()
    }

    fn get_basin_sizes(&self) -> Vec<usize> {
        self.get_low_points_idx()
            .into_iter()
            .map(|idx| self.get_basin(idx).len())
            .collect()
    }

    fn get_basin(&self, idx: usize) -> HashSet<usize> {
        let mut basin = HashSet::new();

        let mut indices_to_check = vec![idx];
        while let Some(to_check) = indices_to_check.pop() {
            if !basin.contains(&to_check) && self.grid[to_check] != 9 {
                basin.insert(to_check);
                let (cx, cy) = self.idx2pos(to_check);
                indices_to_check.extend(self.get_neighbours_idx(cx, cy).into_iter());
            }
        }

        basin
    }

    fn pos2idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn idx2pos(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;
        (x, y)
    }
}

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day9.txt")?;

    let heightmap = Heightmap::from_str(&content).unwrap();
    let risk_level = heightmap.get_risk_level_sum();
    println!("day9 part1 = {}", risk_level);

    let product = heightmap
        .get_3_largest_basins()
        .into_iter()
        .product::<usize>();
    println!("day9 part2 = {}", product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basin() {
        let data = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let heightmap = Heightmap::from_str(data).unwrap();
        assert_eq!(3, heightmap.get_basin(1).len());
        assert_eq!(9, heightmap.get_basin(9).len());
        assert_eq!(14, heightmap.get_basin(22).len());
        assert_eq!(9, heightmap.get_basin(46).len());

        assert_eq!(
            1134usize,
            heightmap.get_3_largest_basins().into_iter().product()
        );
    }
}
