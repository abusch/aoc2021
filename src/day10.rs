use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day10.txt")?;

    let checker = SyntaxChecker::new();
    let score: u64 = content
        .lines()
        .filter_map(|line| {
            checker
                .line_status(line)
                .into_corrupted()
                .map(|s| checker.corruption_score(s))
        })
        .sum();

    println!("day10 part1 = {}", score);

    let mut scores: Vec<u64> = content
        .lines()
        .filter_map(|line| {
            checker
                .line_status(line)
                .into_incomplete()
                .map(|s| checker.incomplete_score(&s))
        })
        .collect();
    scores.sort_unstable();
    let n = scores.len();
    let middle_score = scores[n / 2];

    println!("day10 part2 = {}", middle_score);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum LineStatus {
    Corrupted(char),
    Incomplete(String),
}

impl LineStatus {
    fn into_corrupted(self) -> Option<char> {
        match self {
            LineStatus::Corrupted(c) => Some(c),
            LineStatus::Incomplete(_) => None,
        }
    }

    fn into_incomplete(self) -> Option<String> {
        match self {
            LineStatus::Corrupted(_) => None,
            LineStatus::Incomplete(s) => Some(s),
        }
    }
}

struct Chunk {
    opening: char,
    closing: char,
    corrupt_score: u64,
    incomplete_score: u64,
}

impl Chunk {
    fn new(opening: char, closing: char, corrupt_score: u64, incomplete_score: u64) -> Self {
        Self {
            opening,
            closing,
            corrupt_score,
            incomplete_score,
        }
    }

    /// Get a reference to the chunk's opening.
    fn opening(&self) -> char {
        self.opening
    }

    /// Get a reference to the chunk's closing.
    fn closing(&self) -> char {
        self.closing
    }

    /// Get a reference to the chunk's corrupt score.
    fn corrupt_score(&self) -> u64 {
        self.corrupt_score
    }

    /// Get a reference to the chunk's incomplete score.
    fn incomplete_score(&self) -> u64 {
        self.incomplete_score
    }
}

struct SyntaxChecker {
    syntax: Box<[Chunk]>,
}

impl SyntaxChecker {
    pub fn new() -> Self {
        let syntax = vec![
            Chunk::new('(', ')', 3, 1),
            Chunk::new('[', ']', 57, 2),
            Chunk::new('{', '}', 1197, 3),
            Chunk::new('<', '>', 25137, 4),
        ];
        Self {
            syntax: syntax.into_boxed_slice(),
        }
    }

    fn line_status(&self, line: &str) -> LineStatus {
        let mut stack = vec![];

        for c in line.chars() {
            if self.is_opening(c) {
                stack.push(c);
            } else if self.is_closing(c) {
                let matching = stack.pop().expect("invalid line");
                if !self.is_pair(matching, c) {
                    return LineStatus::Corrupted(c);
                }
            }
        }

        LineStatus::Incomplete(String::from_iter(
            stack.into_iter().rev().map(|c| self.matching_char(c)),
        ))
    }

    fn is_opening(&self, c: char) -> bool {
        self.get_chunk_from_opening(c).is_some()
    }

    fn is_closing(&self, c: char) -> bool {
        self.get_chunk_from_closing(c).is_some()
    }

    fn is_pair(&self, c: char, d: char) -> bool {
        self.syntax
            .iter()
            .any(|chunk| chunk.opening() == c && chunk.closing() == d)
    }

    fn matching_char(&self, c: char) -> char {
        self.get_chunk_from_opening(c)
            .map(Chunk::closing)
            .expect("unknown opening char")
    }

    fn corruption_score(&self, c: char) -> u64 {
        self.get_chunk_from_closing(c)
            .map(Chunk::corrupt_score)
            .unwrap_or(0)
    }

    fn incomplete_score(&self, s: &str) -> u64 {
        s.chars().fold(0u64, |score, c| {
            let char_score = self
                .get_chunk_from_closing(c)
                .map(Chunk::incomplete_score)
                .unwrap_or(0);
            score * 5 + char_score
        })
    }

    fn get_chunk_from_opening(&self, c: char) -> Option<&Chunk> {
        self.syntax.iter().find(|chunk| chunk.opening() == c)
    }
    fn get_chunk_from_closing(&self, c: char) -> Option<&Chunk> {
        self.syntax.iter().find(|chunk| chunk.closing() == c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted() {
        let checker = SyntaxChecker::new();
        assert_eq!(
            LineStatus::Incomplete("".to_string()),
            checker.line_status("[<>({}){}[([])<>]]")
        );
        assert_eq!(
            LineStatus::Corrupted('}'),
            checker.line_status("{([(<{}[<>[]}>{[]{[(<()>")
        );
    }

    #[test]
    fn test_incomplete() {
        let checker = SyntaxChecker::new();
        assert_eq!(
            Some("}}]])})]".to_string()),
            checker
                .line_status("[({(<(())[]>[[{[]{<()<>>")
                .into_incomplete()
        );
        assert_eq!(
            Some("])}>".to_string()),
            checker
                .line_status("<{([{{}}[<[[[<>{}]]]>[]]")
                .into_incomplete()
        );
        assert_eq!(288957, checker.incomplete_score("}}]])})]"));
        assert_eq!(5566, checker.incomplete_score(")}>]})"));
    }
}
