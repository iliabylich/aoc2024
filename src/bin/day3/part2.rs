use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug)]
enum InsKind {
    Mul { lhs: usize, rhs: usize },
    Do,
    Dont,
}

#[derive(Debug)]
struct Ins {
    starts_at: usize,
    kind: InsKind,
}

fn solve(input: &str) -> usize {
    let muls = Regex::new(r#"mul\((\d+),(\d+)\)"#)
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let starts_at = c.get(0).unwrap().start();
            let lhs = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let rhs = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
            Ins {
                starts_at,
                kind: InsKind::Mul { lhs, rhs },
            }
        })
        .collect::<Vec<_>>();

    let dos = Regex::new(r#"do\(\)"#)
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let starts_at = c.get(0).unwrap().start();
            Ins {
                starts_at,
                kind: InsKind::Do,
            }
        })
        .collect::<Vec<_>>();

    let donts = Regex::new(r#"don't\(\)"#)
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let starts_at = c.get(0).unwrap().start();
            Ins {
                starts_at,
                kind: InsKind::Dont,
            }
        })
        .collect::<Vec<_>>();

    let mut insns = Vec::from_iter(muls.into_iter().chain(dos).chain(donts));

    insns.sort_unstable_by_key(|e| e.starts_at);

    let mut enabled = true;
    let mut out = 0;

    for insn in insns {
        match insn.kind {
            InsKind::Mul { lhs, rhs } if enabled => out += lhs * rhs,
            InsKind::Do => enabled = true,
            InsKind::Dont => enabled = false,
            _ => {}
        }
    }

    out
}

#[test]
fn test() {
    let input = include_str!("input2_test.txt");
    let output = solve(input);
    assert_eq!(output, 48);
}
