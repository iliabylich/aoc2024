fn main() {
    let input = include_bytes!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    RightBottom,
    Bottom,
    LeftBottom,
    Left,
    LeftTop,
    Top,
    RightTop,
}

impl Direction {
    fn all() -> [Direction; 8] {
        [
            Self::Right,
            Self::RightBottom,
            Self::Bottom,
            Self::LeftBottom,
            Self::Left,
            Self::LeftTop,
            Self::Top,
            Self::RightTop,
        ]
    }

    fn apply(self, start: Point) -> Word {
        let base = Word([start, start, start, start]);
        match self {
            Direction::Right => base.add0(0, 1),
            Direction::RightBottom => base.add0(1, 1),
            Direction::Bottom => base.add0(1, 0),
            Direction::LeftBottom => base.add0(1, -1),
            Direction::Left => base.add0(0, -1),
            Direction::LeftTop => base.add0(-1, -1),
            Direction::Top => base.add0(-1, 0),
            Direction::RightTop => base.add0(-1, 1),
        }
    }
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
struct Word([Point; 4]);

impl Word {
    fn add(
        self,
        d1: (isize, isize),
        d2: (isize, isize),
        d3: (isize, isize),
        d4: (isize, isize),
    ) -> Self {
        let [p1, p2, p3, p4] = self.0;
        Self([
            p1.add(d1.0, d1.1),
            p2.add(d2.0, d2.1),
            p3.add(d3.0, d3.1),
            p4.add(d4.0, d4.1),
        ])
    }

    fn add0(self, drow: isize, dcol: isize) -> Self {
        self.add(
            (0, 0),
            (drow, dcol),
            (drow * 2, dcol * 2),
            (drow * 3, dcol * 3),
        )
    }

    fn materialize(self, matrix: &Matrix<'_>) -> Option<[u8; 4]> {
        let [p1, p2, p3, p4] = self.0;
        Some([
            p1.materialize(matrix)?,
            p2.materialize(matrix)?,
            p3.materialize(matrix)?,
            p4.materialize(matrix)?,
        ])
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
            for dir in Direction::all() {
                let word = dir.apply(start);
                if let Some(xmas) = word.materialize(&matrix) {
                    if xmas == [b'X', b'M', b'A', b'S'] {
                        out += 1
                    }
                }
            }
        }
    }

    out
}

#[test]
fn test() {
    let input = include_bytes!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 18);
}
