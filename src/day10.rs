use anyhow::Result;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day10.txt")?;

    let score: u64 = content
        .lines()
        .filter_map(|line| line_status(line).as_corrupted().map(corruption_score))
        .sum();

    println!("day10 part1 = {}", score);

    let mut scores: Vec<u64> = content
        .lines()
        // filter out corrupted lines
        .filter_map(|line| {
            line_status(line)
                .as_incomplete()
                .map(|s| incomplete_score(&s))
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
    fn as_corrupted(self) -> Option<char> {
        match self {
            LineStatus::Corrupted(c) => Some(c),
            LineStatus::Incomplete(_) => None,
        }
    }

    fn as_incomplete(self) -> Option<String> {
        match self {
            LineStatus::Corrupted(_) => None,
            LineStatus::Incomplete(s) => Some(s),
        }
    }
}

fn line_status(line: &str) -> LineStatus {
    let mut stack = vec![];

    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let matching = stack.pop().expect("invalid line");
            if !is_pair(matching, c) {
                return LineStatus::Corrupted(c);
            }
        }
    }

    LineStatus::Incomplete(String::from_iter(
        stack.into_iter().rev().map(matching_char),
    ))
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn is_closing(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}

fn is_pair(c: char, d: char) -> bool {
    matches!((c, d), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

fn matching_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("should not happen"),
    }
}

fn corruption_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_score(s: &str) -> u64 {
    s.chars().fold(0u64, |score, c| {
        let char_score = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("should not happen"),
        };
        score * 5 + char_score
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted() {
        assert_eq!(
            LineStatus::Incomplete("".to_string()),
            line_status("[<>({}){}[([])<>]]")
        );
        assert_eq!(
            LineStatus::Corrupted('}'),
            line_status("{([(<{}[<>[]}>{[]{[(<()>")
        );
    }

    #[test]
    fn test_incomplete() {
        assert_eq!(
            Some("}}]])})]".to_string()),
            line_status("[({(<(())[]>[[{[]{<()<>>").as_incomplete()
        );
        assert_eq!(
            Some("])}>".to_string()),
            line_status("<{([{{}}[<[[[<>{}]]]>[]]").as_incomplete()
        );
        assert_eq!(288957, incomplete_score("}}]])})]"));
        assert_eq!(5566, incomplete_score(")}>]})"));
    }
}
