fn main() {
    let input = include_str!("input2.txt");
    let output = solve(input);
    println!("{}", output);
}

fn solve(input: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let l = l.parse::<usize>().unwrap();
        let r = r.parse::<usize>().unwrap();
        left.push(l);
        right.push(r);
    }
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[test]
fn test() {
    let input = include_str!("input1.txt");
    let output = solve(input);
    assert_eq!(output, 11);
}
