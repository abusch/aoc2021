use std::iter::from_fn;

use anyhow::{Context, Result};
use internment::Intern;
use itertools::Itertools;
use petgraph::prelude::*;

type Cave = Intern<String>;
type Caves = UnGraphMap<Cave, ()>;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day12.txt")?;

    let mut graph: Caves = UnGraphMap::new();
    // build the graph from the data
    // note: we intern the strings to not have to deal with lifetimes, and to avoid too many
    // allocations
    for line in content.lines() {
        let (from, to) = line.split_once('-').context("Invalid edge")?;
        graph.add_edge(Intern::from(from), Intern::from(to), ());
    }

    let paths = all_paths(
        &graph,
        Intern::from("start"),
        Intern::from("end"),
        allow_cave1,
    )
    .count();
    println!("day12 part1 = {}", paths);

    let paths = all_paths(
        &graph,
        Intern::from("start"),
        Intern::from("end"),
        allow_cave2,
    )
    .count();
    println!("day12 part2 = {}", paths);

    Ok(())
}

// adapted from https://github.com/petgraph/petgraph/blob/master/src/algo/simple_paths.rs#L36
pub fn all_paths<'a, F>(
    graph: &'a Caves,
    from: Cave,
    to: Cave,
    allow_cave: F,
) -> impl Iterator<Item = Vec<Cave>> + 'a
where
    F: Fn(Cave, &[Cave]) -> bool + 'a,
{
    // list of visited nodes (i.e. the current path so far)
    let mut visited: Vec<Cave> = Vec::from_iter(Some(from));
    // list of children of currently exploring path nodes,
    // last elem is list of children of last visited node
    // Note: each element is *an iterator*, i.e. it keeps its state of where it's at even when we
    // explore the next level.
    let mut stack = vec![graph.neighbors(from)];

    from_fn(move || {
        while let Some(children) = stack.last_mut() {
            if let Some(child) = children.next() {
                if child == to {
                    // We reached the end node: return the current path as the next value of the
                    // iterator
                    let path = visited
                        .iter()
                        .cloned()
                        .chain(Some(to))
                        .collect::<Vec<Cave>>();
                    return Some(path);
                } else if allow_cave(child, &visited) {
                    // Keep exploring paths (if we're allowed to)
                    visited.push(child);
                    stack.push(graph.neighbors(child));
                }
            } else {
                // we've exhausted all the children of the current node: backtrack to the previous
                // last visited node. This is the equivalent of a "return" if this function was
                // written using recursion.
                stack.pop();
                visited.pop();
            }
        }
        None
    })
}

fn allow_cave1(cave: Cave, visited: &[Cave]) -> bool {
    !is_small_cave(cave) || !visited.contains(&cave)
}

fn allow_cave2(cave: Cave, visited: &[Cave]) -> bool {
    !is_small_cave(cave)
        || !visited.contains(&cave)
        || (!is_start(cave) && !has_duplicate_small_cave(visited))
}

fn is_small_cave(c: Cave) -> bool {
    c.as_ref().bytes().next().unwrap().is_ascii_lowercase()
}

fn is_start(c: Cave) -> bool {
    c == Intern::from("start")
}

fn has_duplicate_small_cave(list: &[Cave]) -> bool {
    let start = Intern::from("start");
    let end = Intern::from("end");
    let counts = list
        .iter()
        .copied()
        // only keep small caves that neither start nor end
        .filter(|s| is_small_cave(*s) && *s != start && *s != end)
        .counts();
    // dbg!(&counts);
    counts.values().contains(&2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let mut graph = UnGraphMap::new();
        for line in content.lines() {
            let (from, to) = line.split_once('-').context("Invalid edge").unwrap();
            graph.add_edge(Intern::from(from), Intern::from(to), ());
        }

        let paths = all_paths(
            &graph,
            Intern::from("start"),
            Intern::from("end"),
            allow_cave1,
        )
        .count();
        assert_eq!(10, paths);

        let paths = all_paths(
            &graph,
            Intern::from("start"),
            Intern::from("end"),
            allow_cave2,
        )
        .count();

        assert_eq!(36, paths);
    }

    #[test]
    fn has_duplicates() {
        let list = vec![
            Intern::from("start"),
            Intern::from("A"),
            Intern::from("b"),
            Intern::from("A"),
            Intern::from("c"),
            Intern::from("A"),
            Intern::from("b"),
            Intern::from("A"),
            Intern::from("end"),
        ];
        assert!(has_duplicate_small_cave(&list));

        let list = vec![
            Intern::from("start"),
            Intern::from("A"),
            Intern::from("b"),
            Intern::from("A"),
            Intern::from("c"),
            Intern::from("A"),
            Intern::from("A"),
            Intern::from("end"),
        ];
        assert!(!has_duplicate_small_cave(&list));
    }
}
