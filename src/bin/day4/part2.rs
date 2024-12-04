fn main() {
    let input = include_bytes!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn add(self, drow: isize, dcol: isize) -> Self {
        Self {
            row: self.row + drow,
            col: self.col + dcol,
        }
    }

    fn materialize(self, matrix: &Matrix<'_>) -> Option<u8> {
        if self.row < 0 || self.col < 0 {
            return None;
        }
        let row = self.row as usize;
        let col = self.col as usize;
        matrix.lines.get(row)?.get(col).copied()
    }
}

#[derive(Debug, Clone, Copy)]
struct Square(Point);

impl Square {
    fn top_left(self) -> Point {
        self.0
    }
    fn top_right(self) -> Point {
        self.0.add(0, 2)
    }
    fn center(self) -> Point {
        self.0.add(1, 1)
    }
    fn bottom_left(self) -> Point {
        self.0.add(2, 0)
    }
    fn bottom_right(self) -> Point {
        self.0.add(2, 2)
    }

    fn diags(self) -> [Diag; 2] {
        [
            Diag([self.top_left(), self.center(), self.bottom_right()]),
            Diag([self.top_right(), self.center(), self.bottom_left()]),
        ]
    }
    fn matches(self, matrix: &Matrix<'_>) -> bool {
        let [diag1, diag2] = self.diags();

        diag1.matches(matrix) && diag2.matches(matrix)
    }
}

struct Diag([Point; 3]);

impl Diag {
    fn materialize(self, matrix: &Matrix<'_>) -> Option<[u8; 3]> {
        let [p1, p2, p3] = self.0;
        Some([
            p1.materialize(matrix)?,
            p2.materialize(matrix)?,
            p3.materialize(matrix)?,
        ])
    }

    fn matches(self, matrix: &Matrix<'_>) -> bool {
        if let Some(bytes) = self.materialize(matrix) {
            return bytes == [b'M', b'A', b'S'] || bytes == [b'S', b'A', b'M'];
        }
        false
    }
}

#[derive(Debug)]
struct Matrix<'a> {
    lines: Vec<&'a [u8]>,
    rows_count: usize,
    cols_count: usize,
}

fn solve(input: &[u8]) -> usize {
    let mut out = 0;

    let lines = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let matrix = Matrix {
        rows_count: lines.len(),
        cols_count: lines[0].len(),
        lines,
    };
    for row in 0..matrix.rows_count {
        for col in 0..matrix.cols_count {
            let start = Point {
                row: row as isize,
                col: col as isize,
            };
            let square = Square(start);
            if square.matches(&matrix) {
                out += 1;
            }
        }
    }

    out
}

#[test]
fn test() {
    let input = include_bytes!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 9);
}
