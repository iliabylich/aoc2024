use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

type Seq = [u8; 5];

#[derive(Debug)]
struct LocksAndKeys {
    locks: HashSet<Seq>,
    keys: HashSet<Seq>,
}

fn parse_rectangle(input: &str) -> (usize, usize, Vec<Vec<bool>>) {
    let mut out: Vec<Vec<bool>> = vec![];
    for row in input.lines() {
        let row_as_marks = row
            .bytes()
            .map(|b| {
                assert!(b == b'#' || b == b'.');
                b == b'#'
            })
            .collect::<Vec<_>>();
        out.push(row_as_marks);
    }

    let rows_count = out.len();
    let estimated_cols_count = out[0].len();

    for row in out.iter() {
        assert_eq!(row.len(), estimated_cols_count);
    }
    let cols_count = estimated_cols_count;

    (rows_count, cols_count, out)
}

impl LocksAndKeys {
    fn parse(input: &str) -> Self {
        let mut locks = HashSet::new();
        let mut keys = HashSet::new();

        for case in input.split("\n\n") {
            let (rows, cols, matrix) = parse_rectangle(case);
            assert_eq!(cols, 5);

            // just to make sure we don't parse garbage for part 2
            if matrix[0][0] {
                // goes down, it's a lock
                let mut heights = [0; 5];
                #[allow(clippy::needless_range_loop)]
                for col in 0..cols {
                    let mut height = 0_u8;
                    for row in 0..rows {
                        if matrix[row][col] {
                            height += 1;
                        } else {
                            break;
                        }
                    }
                    heights[col] = height.checked_sub(1).unwrap();
                }
                locks.insert(heights);
            } else {
                // goes up, it's a key
                let mut heights = [0; 5];
                #[allow(clippy::needless_range_loop)]
                for col in 0..cols {
                    let mut height = 0_u8;
                    for row in (0..rows).rev() {
                        if matrix[row][col] {
                            height += 1;
                        } else {
                            break;
                        }
                    }
                    heights[col] = height.checked_sub(1).unwrap();
                }
                keys.insert(heights);
            };
        }

        Self { locks, keys }
    }

    fn find_pairs(self) -> Vec<(Seq, Seq)> {
        let mut out = vec![];

        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                const TARGET: u8 = 5;
                if lock.iter().zip(key).all(|(l, k)| *l + *k <= TARGET) {
                    out.push((*lock, *key));
                }
            }
        }

        out
    }
}

fn solve(input: &str) -> u64 {
    let locks_and_keys = LocksAndKeys::parse(input);
    println!("{:?}", locks_and_keys);

    let matches = locks_and_keys.find_pairs();

    matches.len() as u64
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, 3);
}
