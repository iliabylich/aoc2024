fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

fn solve(input: &str) -> usize {
    let (patterns, lines) = input.trim().split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();

    let lines = lines.lines().collect::<Vec<_>>();

    let mut out = 0;
    for line in lines {
        let mut dp = vec![0; 1000];

        for offset in (0..=line.len() - 1).rev() {
            let mut total = 0;
            let trailing = &line[offset..line.len()];
            for pattern in patterns.iter() {
                if trailing.starts_with(pattern) {
                    let rest_starts_at = offset + pattern.len();
                    let rest = &line[rest_starts_at..];

                    total += dp[rest_starts_at];

                    if rest.is_empty() {
                        total += 1;
                    }
                }
            }
            dp[offset] = total
        }

        out += dp[0];
    }

    out
}

#[test]
fn test1() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 16);
}
