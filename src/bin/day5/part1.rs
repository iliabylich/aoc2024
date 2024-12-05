fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug)]
struct OrderingMap {
    map: [[bool; 100]; 100],
}

impl OrderingMap {
    fn new(input: &str) -> Self {
        let mut map = [[false; 100]; 100];
        for line in input.lines() {
            let (before, after) = line.split_once('|').unwrap();
            let before = before.parse::<usize>().unwrap();
            let after = after.parse::<usize>().unwrap();
            map[before][after] = true;
        }
        Self { map }
    }

    fn is_valid(&self, before: usize, after: usize) -> bool {
        self.map[before][after]
    }
}

#[derive(Debug)]
struct Pages {
    pages: Vec<usize>,
}

impl Pages {
    fn new(line: &str) -> Self {
        Self {
            pages: line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn is_valid(&self, map: &OrderingMap) -> bool {
        self.pages
            .iter()
            .zip(self.pages.iter().skip(1))
            .all(|(before, after)| map.is_valid(*before, *after))
    }

    fn middle(&self) -> usize {
        assert!(self.pages.len() % 2 != 0);
        self.pages[self.pages.len() / 2]
    }
}

fn solve(input: &str) -> usize {
    let (ordering, pages) = input.trim().split_once("\n\n").unwrap();

    let ordering = OrderingMap::new(ordering);

    let mut out = 0;
    for pages in pages.lines() {
        let pages = Pages::new(pages);
        if pages.is_valid(&ordering) {
            out += pages.middle();
        }
    }

    out
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 143);
}
