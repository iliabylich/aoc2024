use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    base: Position,
    dir: Position,
}

impl Ray {
    fn points(self, matrix: &Matrix) -> Vec<Position> {
        let drow = self.dir.row - self.base.row;
        let dcol = self.dir.col - self.base.col;

        let mut out = vec![];

        let mut current = self.dir;
        loop {
            current = current.add(drow, dcol);
            if !matrix.contains(current) {
                break;
            }
            out.push(current);
        }

        out
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn add(self, drow: isize, dcol: isize) -> Position {
        Position {
            row: self.row + drow,
            col: self.col + dcol,
        }
    }
}

#[derive(Debug)]
struct Matrix {
    rows_count: usize,
    cols_count: usize,
    clusters: HashMap<u8, Vec<Position>>,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut rows_count = 0;
        let mut cols_count = 0;
        let mut clusters: HashMap<u8, Vec<Position>> = HashMap::new();

        for (row, line) in input.trim().lines().enumerate() {
            for (col, b) in line.bytes().enumerate() {
                rows_count = std::cmp::max(rows_count, row);
                cols_count = std::cmp::max(cols_count, col);

                if b.is_ascii_alphanumeric() {
                    let pos = Position {
                        row: row as isize,
                        col: col as isize,
                    };
                    clusters.entry(b).or_default().push(pos);
                }
            }
        }

        Self {
            rows_count: rows_count + 1,
            cols_count: cols_count + 1,
            clusters,
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

    for (_, cluster) in matrix.clusters.iter() {
        for a1 in cluster.iter() {
            for a2 in cluster.iter() {
                if a1 == a2 {
                    continue;
                }
                uniq.insert(*a1);
                uniq.insert(*a2);

                for (base, dir) in [(*a1, *a2), (*a2, *a1)] {
                    let ray = Ray { base, dir };
                    let points = ray.points(&matrix);

                    for point in points {
                        uniq.insert(point);
                    }
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
    assert_eq!(output, 34);
}
