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
        [
            self.sibling_up(),
            self.sibling_down(),
            self.sibling_left(),
            self.sibling_right(),
        ]
    }

    fn sibling_up(self) -> Loc {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn sibling_down(self) -> Loc {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn sibling_left(self) -> Loc {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn sibling_right(self) -> Loc {
        Self {
            row: self.row,
            col: self.col + 1,
        }
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

    fn sides(&self) -> (HashSet<Side>, HashSet<Side>) {
        let mut hsides = HashSet::new();
        let mut vsides = HashSet::new();

        for loc in self.locations.iter() {
            let Loc { row, col } = *loc;
            let row = row as usize;
            let col = col as usize;

            let sibling = loc.sibling_up();
            if !self.locations.contains(&sibling) {
                hsides.insert(Side {
                    row_or_col: row,
                    starts_at: col,
                    ends_at: col + 1,
                    kind: SideKind::Up,
                });
            }

            let sibling = loc.sibling_down();
            if !self.locations.contains(&sibling) {
                hsides.insert(Side {
                    row_or_col: row + 1,
                    starts_at: col,
                    ends_at: col + 1,
                    kind: SideKind::Down,
                });
            }

            let sibling = loc.sibling_left();
            if !self.locations.contains(&sibling) {
                vsides.insert(Side {
                    row_or_col: col,
                    starts_at: row,
                    ends_at: row + 1,
                    kind: SideKind::Left,
                });
            }

            let sibling = loc.sibling_right();
            if !self.locations.contains(&sibling) {
                vsides.insert(Side {
                    row_or_col: col + 1,
                    starts_at: row,
                    ends_at: row + 1,
                    kind: SideKind::Right,
                });
            }
        }

        optimize(&mut hsides);
        optimize(&mut vsides);

        (hsides, vsides)
    }

    fn sides_count(&self) -> usize {
        let (hsides, vsides) = self.sides();

        hsides.len() + vsides.len()
    }

    fn price(&self) -> usize {
        self.area() * self.sides_count()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Side {
    starts_at: usize,
    ends_at: usize,
    row_or_col: usize,
    kind: SideKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum SideKind {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    fn merge(self, other: Self) -> Option<Self> {
        if self.row_or_col != other.row_or_col || self.kind != other.kind {
            return None;
        }
        if self.ends_at == other.starts_at {
            return Some(Self {
                row_or_col: self.row_or_col,
                kind: self.kind,
                starts_at: self.starts_at,
                ends_at: other.ends_at,
            });
        }
        if other.ends_at == self.starts_at {
            return Some(Self {
                row_or_col: self.row_or_col,
                kind: self.kind,
                starts_at: other.starts_at,
                ends_at: self.ends_at,
            });
        }

        None
    }
}

fn optimize(sides: &mut HashSet<Side>) {
    loop {
        let mut merge_insn = None;

        'inner: for side1 in sides.iter() {
            for side2 in sides.iter() {
                if let Some(merged) = side1.merge(*side2) {
                    merge_insn = Some((*side1, *side2, merged));
                    break 'inner;
                }
            }
        }

        if let Some((side1, side2, merged)) = merge_insn {
            sides.remove(&side1);
            sides.remove(&side2);
            sides.insert(merged);
        } else {
            break;
        }
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
        println!("sides: {:?}", shape.sides());
        println!("sides_count: {:?}", shape.sides_count());
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
    assert_eq!(output, 1206);
}
