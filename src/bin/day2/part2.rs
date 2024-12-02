fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Dec,
    Inc,
    Invalid,
}

impl Dir {
    fn from_cons(l: usize, r: usize) -> Self {
        match l.cmp(&r) {
            std::cmp::Ordering::Less => Self::Inc,
            std::cmp::Ordering::Equal => Self::Invalid,
            std::cmp::Ordering::Greater => Self::Dec,
        }
    }
}

fn is_safe(levels: Vec<usize>) -> bool {
    let starting_dir = Dir::from_cons(levels[0], levels[1]);
    if starting_dir == Dir::Invalid {
        return false;
    }

    for (prev, next) in levels.iter().zip(levels.iter().skip(1)) {
        let dir = Dir::from_cons(*prev, *next);
        if dir != starting_dir {
            return false;
        }
        let diff = prev.abs_diff(*next);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn is_safe_if_removing_one_level(line: &str) -> bool {
    let levels = line
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for idx_to_remove in 0..levels.len() {
        let mut attempt = levels.clone();
        attempt.remove(idx_to_remove);
        if is_safe(attempt) {
            return true;
        }
    }

    false
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_safe_if_removing_one_level(line))
        .count()
}

#[test]
fn test() {
    let input = include_str!("input1_test.txt");
    let output = solve(input);
    assert_eq!(output, 4);
}
