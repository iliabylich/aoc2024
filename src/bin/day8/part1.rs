use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn antipositions(self, other: Position) -> (Position, Position) {
        let (drow, dcol) = self.distance(other);
        let drow = drow as isize;
        let dcol = dcol as isize;

        let candidates = [
            self.add(drow, dcol),
            self.add(drow, -dcol),
            self.add(-drow, dcol),
            self.add(-drow, -dcol),
            other.add(drow, dcol),
            other.add(drow, -dcol),
            other.add(-drow, dcol),
            other.add(-drow, -dcol),
        ];

        let mut matching = vec![];
        for candidate in candidates {
            let (drow1, dcol1) = candidate.distance(self);
            let (drow2, dcol2) = candidate.distance(other);
            if (drow1 * 2 == drow2 && dcol1 * 2 == dcol2)
                || (drow2 * 2 == drow1 && dcol2 * 2 == dcol1)
            {
                matching.push(candidate);
            }
        }

        assert_eq!(matching.len(), 2);

        let mut iter = matching.into_iter();

        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        (first, second)
    }

    fn distance(self, other: Position) -> (usize, usize) {
        (self.row.abs_diff(other.row), self.col.abs_diff(other.col))
    }

    fn add(self, drow: isize, dcol: isize) -> Position {
        Position {
            row: self.row + drow,
            col: self.col + dcol,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Mark {
    pos: Position,
    freq: u8,
}

impl Mark {
    fn parse(pos: Position, freq: u8) -> Option<Mark> {
        if freq.is_ascii_alphanumeric() {
            Some(Self { pos, freq })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Matrix {
    marks: Vec<Mark>,
    rows_count: usize,
    cols_count: usize,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut rows_count = 0;
        let mut cols_count = 0;
        let mut marks = vec![];

        for (row, line) in input.trim().lines().enumerate() {
            for (col, b) in line.bytes().enumerate() {
                rows_count = std::cmp::max(rows_count, row);
                cols_count = std::cmp::max(cols_count, col);

                if let Some(mark) = Mark::parse(
                    Position {
                        row: row as isize,
                        col: col as isize,
                    },
                    b,
                ) {
                    marks.push(mark);
                }
            }
        }

        Self {
            marks,
            rows_count: rows_count + 1,
            cols_count: cols_count + 1,
        }
    }

    fn contains(&self, pos: Position) -> bool {
        if pos.row < 0 || pos.col < 0 {
            return false;
        }
        let row = pos.row as usize;
        let col = pos.col as usize;
        row < self.rows_count && col < self.cols_count
    }
}

fn solve(input: &str) -> usize {
    let matrix = Matrix::parse(input);

    let mut uniq = HashSet::new();

    for mark1 in matrix.marks.iter() {
        for mark2 in matrix.marks.iter() {
            if mark1 != mark2 && mark1.freq == mark2.freq {
                let (anti1, anti2) = mark1.pos.antipositions(mark2.pos);

                if matrix.contains(anti1) {
                    uniq.insert(anti1);
                }
                if matrix.contains(anti2) {
                    uniq.insert(anti2);
                }
            }
        }
    }

    uniq.len()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 14);
}
