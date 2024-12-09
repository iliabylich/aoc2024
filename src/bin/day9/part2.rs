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
struct Location {
    starts_at: usize,
    length: usize,
}

#[derive(Debug)]
struct BlockSlot {
    block_idx: usize,
    location: Location,
}

#[derive(Debug)]
struct FreeSlot {
    location: Location,
}

#[derive(Debug)]
struct Filesystem {
    blocks: Vec<Block>,
    max_block_idx: usize,
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

        Self {
            blocks,
            max_block_idx: block_idx - 1,
        }
    }

    fn split(&self) -> (Vec<BlockSlot>, Vec<FreeSlot>) {
        let mut block_slots = vec![];
        let mut free_slots = vec![];
        let mut pos = 0;

        for chunk in self
            .blocks
            .chunk_by(|block1, block2| match (block1, block2) {
                (Block::Free, Block::Free) => true,
                (Block::Free, Block::Used(_)) => false,
                (Block::Used(_), Block::Free) => false,
                (Block::Used(block_idx1), Block::Used(block_idx2)) => block_idx1 == block_idx2,
            })
        {
            let location = Location {
                starts_at: pos,
                length: chunk.len(),
            };
            match chunk.first().unwrap() {
                Block::Free => free_slots.push(FreeSlot { location }),
                Block::Used(block_idx) => block_slots.push(BlockSlot {
                    block_idx: *block_idx,
                    location,
                }),
            }
            pos += chunk.len();
        }

        (block_slots, free_slots)
    }

    fn move_block(
        &mut self,
        block_idx: usize,
        block_slots: &[BlockSlot],
        free_slots: &[FreeSlot],
    ) -> bool {
        let block = block_slots
            .iter()
            .find(|s| s.block_idx == block_idx)
            .unwrap();

        let Some(free_slot) = free_slots.iter().find(|free_slot| {
            free_slot.location.starts_at < block.location.starts_at
                && free_slot.location.length >= block.location.length
        }) else {
            return false;
        };

        let len = block.location.length;
        let free_start = free_slot.location.starts_at;
        let block_start = block.location.starts_at;

        for idx in free_start..free_start + len {
            self.blocks[idx] = Block::Used(block_idx);
        }
        for idx in block_start..block_start + len {
            self.blocks[idx] = Block::Free;
        }

        true
    }

    fn defragment(&mut self) {
        let (mut block_slots, mut free_slots) = self.split();

        for block_idx in (0..=self.max_block_idx).rev() {
            if self.move_block(block_idx, &block_slots, &free_slots) {
                // println!("{}", self);
                (block_slots, free_slots) = self.split();
            }
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
    // println!("{:?}", fs);

    fs.defragment();
    fs.checksum()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 2858);
}
