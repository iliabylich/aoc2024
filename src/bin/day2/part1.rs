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

fn is_safe(line: &str) -> bool {
    let levels = line
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

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
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn solve(input: &str) -> usize {
    input.lines().filter(|line| is_safe(line)).count()
}

#[test]
fn test() {
    let input = include_str!("input1_test.txt");
    let output = solve(input);
    assert_eq!(output, 2);
}
