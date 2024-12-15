fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: isize,
    col: isize,
}

impl Location {
    fn validate(self, matrix: &Matrix) -> Option<(usize, usize)> {
        if self.row < 0 || self.col < 0 {
            return None;
        }
        let row = self.row as usize;
        let col = self.col as usize;
        if row < matrix.rows_count && col < matrix.cols_count {
            Some((row, col))
        } else {
            None
        }
    }

    fn siblings(self) -> [Self; 4] {
        let Self { row, col } = self;
        [
            Self { row: row + 1, col },
            Self { row: row - 1, col },
            Self { row, col: col + 1 },
            Self { row, col: col - 1 },
        ]
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u8>>,
    rows_count: usize,
    cols_count: usize,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut data = vec![];
        for line in input.trim().lines() {
            let row = line.bytes().map(|b| b - b'0').collect::<Vec<_>>();
            data.push(row);
        }
        let rows_count = data.len();
        let cols_count = data[0].len();
        Self {
            data,
            rows_count,
            cols_count,
        }
    }

    fn zeroes(&self) -> Vec<Location> {
        let mut out = vec![];
        for row in 0..self.rows_count {
            for col in 0..self.cols_count {
                if self.data[row][col] == 0 {
                    out.push(Location {
                        row: row as isize,
                        col: col as isize,
                    })
                }
            }
        }
        out
    }

    fn paths_to_nines(&self, start: Location) -> Vec<Vec<Location>> {
        let mut out = vec![];
        let path = vec![start];
        self.extend_path(path, &mut out);
        out
    }

    fn get(&self, loc: Location) -> Option<u8> {
        let (row, col) = loc.validate(self)?;
        Some(self.data[row][col])
    }

    fn extend_path(&self, path: Vec<Location>, out: &mut Vec<Vec<Location>>) {
        let last_loc = *path.last().unwrap();
        let last_value = self.get(last_loc).unwrap();
        if last_value == 9 {
            out.push(path);
            // full path
            return;
        }

        for next_loc in last_loc.siblings() {
            if let Some(next_value) = self.get(next_loc) {
                if next_value == last_value + 1 {
                    let mut deeper = path.clone();
                    deeper.push(next_loc);
                    self.extend_path(deeper, out);
                }
            }
        }
    }
}

fn solve(input: &str) -> usize {
    let matrix = Matrix::parse(input);
    let mut out = 0;
    for start in matrix.zeroes() {
        let paths = matrix.paths_to_nines(start);
        out += paths.len();
    }
    out
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 81);
}
