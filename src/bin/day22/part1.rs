fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Secret(u64);

fn mix(n: u64, other: u64) -> u64 {
    n ^ other
}

fn prune(n: u64) -> u64 {
    n % 16777216
}

impl Secret {
    fn next(self) -> Self {
        let n = self.0;

        let n = prune(mix(n, n * 64));
        let n = prune(mix(n, n / 32));
        let n = prune(mix(n, n * 2048));

        Self(n)
    }
}

fn solve(input: &str) -> u64 {
    let mut secrets = input
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(Secret)
        .collect::<Vec<_>>();

    for _ in 0..2000 {
        for secret in &mut secrets {
            *secret = secret.next();
        }
    }

    secrets.into_iter().map(|secret| secret.0).sum()
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 37327623);
}
