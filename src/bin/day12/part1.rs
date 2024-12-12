use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc {
    row: isize,
    col: isize,
}

impl Loc {
    fn siblings(self) -> [Loc; 4] {
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
            let row = line.bytes().collect::<Vec<_>>();
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

    fn get(&self, loc: Loc) -> Option<(usize, usize, u8)> {
        if loc.row < 0 || loc.col < 0 {
            return None;
        }
        let row = loc.row as usize;
        let col = loc.col as usize;
        if row >= self.rows_count || col >= self.cols_count {
            return None;
        }
        Some((row, col, self.data[row][col]))
    }

    fn shapes(&self) -> Vec<Shape> {
        let mut remaining = HashSet::new();
        for row in 0..self.rows_count {
            for col in 0..self.cols_count {
                remaining.insert(Loc {
                    row: row as isize,
                    col: col as isize,
                });
            }
        }

        let mut out = vec![];

        while !remaining.is_empty() {
            let start = *remaining.iter().next().unwrap();
            remaining.remove(&start);
            let (_, _, pattern) = self.get(start).unwrap();

            let mut queue = VecDeque::new();
            queue.push_back(start);

            let mut shape = Shape {
                locations: HashSet::from([start]),
            };

            while let Some(current) = queue.pop_front() {
                for sibling in current.siblings() {
                    if let Some((_, _, byte)) = self.get(sibling) {
                        if remaining.contains(&sibling) && byte == pattern {
                            queue.push_back(sibling);
                            remaining.remove(&sibling);
                            shape.locations.insert(sibling);
                        }
                    }
                }
            }

            out.push(shape);
        }

        out
    }
}

#[derive(Debug)]
struct Shape {
    locations: HashSet<Loc>,
}

impl Shape {
    fn area(&self) -> usize {
        self.locations.len()
    }

    fn perimeter(&self) -> usize {
        let mut out = 0;

        for loc in self.locations.iter() {
            for sibling in loc.siblings() {
                if !self.locations.contains(&sibling) {
                    out += 1;
                }
            }
        }

        out
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[allow(dead_code)]
fn print_shapes(shapes: &[Shape], matrix: &Matrix) {
    for shape in shapes {
        let random_loc = *shape.locations.iter().next().unwrap();
        println!("{}", matrix.get(random_loc).unwrap().2 as char);
        println!(
            "{:?}",
            shape
                .locations
                .iter()
                .map(|loc| format!(
                    "({},{},{})",
                    loc.row,
                    loc.col,
                    matrix.get(*loc).unwrap().2 as char
                ))
                .collect::<Vec<_>>()
        );
        println!("area: {:?}", shape.area());
        println!("perimeter: {:?}", shape.perimeter());
    }
}

fn solve(input: &str) -> usize {
    let matrix = Matrix::parse(input);
    let shapes = matrix.shapes();

    // print_shapes(&shapes, &matrix);

    shapes.into_iter().map(|shape| shape.price()).sum()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 1930);
}
