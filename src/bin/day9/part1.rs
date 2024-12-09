fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Free,
    Used(usize),
}

#[derive(Debug)]
struct Filesystem {
    blocks: Vec<Block>,
}

impl Filesystem {
    fn parse(input: &str) -> Self {
        #[derive(Clone, Copy)]
        enum State {
            File,
            FreeSpace,
        }
        const STATES: [State; 2] = [State::File, State::FreeSpace];

        let mut blocks = vec![];
        let mut block_idx = 0;

        for (byte, state) in input.trim().bytes().zip(STATES.into_iter().cycle()) {
            let n = byte - b'0';
            let block = match state {
                State::File => {
                    let block = Block::Used(block_idx);
                    block_idx += 1;
                    block
                }
                State::FreeSpace => Block::Free,
            };

            for _ in 0..n {
                blocks.push(block);
            }
        }

        Self { blocks }
    }

    fn defragment(&mut self) {
        let skip_left = |left: &mut usize, blocks: &[Block]| {
            while *left < blocks.len() && !matches!(blocks[*left], Block::Free) {
                *left += 1;
            }
        };
        let skip_right = |right: &mut usize, blocks: &[Block]| {
            while *right > 0 && matches!(blocks[*right], Block::Free) {
                *right -= 1
            }
        };

        let mut left = 0;
        let mut right = self.blocks.len() - 1;

        skip_left(&mut left, &self.blocks);
        skip_right(&mut right, &self.blocks);

        while left < right {
            self.blocks.swap(left, right);
            // println!("{}", self);

            skip_left(&mut left, &self.blocks);
            skip_right(&mut right, &self.blocks);
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, block)| match block {
                Block::Free => None,
                Block::Used(value) => Some(pos * *value),
            })
            .sum()
    }
}

impl std::fmt::Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for block in self.blocks.iter() {
            match block {
                Block::Free => buf.push('.'),
                Block::Used(idx) => buf.push_str(&format!("{}", idx)),
            }
        }
        write!(f, "{}", buf)
    }
}

fn solve(input: &str) -> usize {
    let mut fs = Filesystem::parse(input);
    // println!("{}", fs);

    fs.defragment();
    fs.checksum()
}

#[test]
fn test() {
    let input = include_str!("input1_test.txt");
    let output = solve(input);
    assert_eq!(output, 1928);
}
