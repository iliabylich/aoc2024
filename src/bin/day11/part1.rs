fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Stone(usize);

impl Stone {
    fn blink(self) -> (Stone, Option<Stone>) {
        if self.0 == 0 {
            (Stone(1), None)
        } else if format!("{}", self.0).len() % 2 == 0 {
            let s = format!("{}", self.0);
            let (l, r) = s.split_at(s.len() / 2);
            (Stone(l.parse().unwrap()), Some(Stone(r.parse().unwrap())))
        } else {
            (Stone(self.0 * 2024), None)
        }
    }
}

fn solve(input: &str) -> usize {
    let mut stones = vec![];
    for line in input.trim().split(' ') {
        stones.push(Stone(line.parse().unwrap()));
    }

    let mut iteration = stones;
    for _ in 0..25 {
        let mut next = vec![];
        for stone in iteration {
            let (s1, s2) = stone.blink();
            next.push(s1);
            if let Some(s2) = s2 {
                next.push(s2);
            }
        }
        iteration = next;
    }

    iteration.len()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 55312);
}
