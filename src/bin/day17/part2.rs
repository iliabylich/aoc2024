use std::collections::HashMap;

fn main() {
    let (_cycle_len, map) = find_cycle();

    const TAPE: [usize; 16] = [2, 4, 1, 6, 7, 5, 4, 6, 1, 4, 5, 5, 0, 3, 3, 0];

    fn recurse(rest: &[usize], n: usize, map: &HashMap<usize, Vec<usize>>) -> Option<usize> {
        if rest.is_empty() {
            let mut buf = vec![];
            manual_sim(n, &mut buf);

            if buf == TAPE {
                return Some(n);
            } else {
                println!("Skipping {n}, it's malformed");
                return None;
            }
        }

        for next in map.get(rest.last().unwrap()).unwrap() {
            let mut it = *next;
            let jumps = if let Some(ks) = (n * 8).checked_sub(it) {
                ks / 1024
            } else {
                0
            };
            it += jumps * 1024;
            while it < n * 8 {
                it += 1024;
            }
            if it / 8 == n {
                // try
                if let Some(deeper) = recurse(&rest[0..rest.len() - 1], it, map) {
                    return Some(deeper);
                }
            }
        }

        None
    }

    let mut found = None;
    for last in map.get(TAPE.last().unwrap()).unwrap() {
        if let Some(n) = recurse(&TAPE[0..TAPE.len() - 1], *last, &map) {
            found = Some(n);
            break;
        }
    }

    assert_eq!(found, Some(90938893795561));

    let input = include_str!("input.txt");
    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = Registers::parse(registers);
    let program = Program::parse(program);
    registers.a = found.unwrap();

    let output = eval(registers, program);
    assert_eq!(output, TAPE);
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

impl Registers {
    fn parse(input: &str) -> Self {
        let (a, rest) = input.split_once('\n').unwrap();
        let (b, c) = rest.split_once('\n').unwrap();

        let a = a.strip_prefix("Register A: ").unwrap();
        let b = b.strip_prefix("Register B: ").unwrap();
        let c = c.strip_prefix("Register C: ").unwrap();

        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    instruction_pointer: usize,
    tape: Vec<usize>,
}

impl Program {
    fn parse(input: &str) -> Self {
        let tape = input.trim().strip_prefix("Program: ").unwrap();

        let nums = tape
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            instruction_pointer: 0,
            tape: nums.clone(),
        }
    }

    fn next_insn(&self) -> Option<Instruction> {
        let l = *self.tape.get(self.instruction_pointer)?;
        let r = *self.tape.get(self.instruction_pointer + 1)?;

        Some(Instruction::parse(l, r))
    }
}

#[derive(Debug)]
enum InstructionOpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl InstructionOpCode {
    fn parse(b: usize) -> Self {
        match b {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("invalid insn opcode: {b}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: InstructionOpCode,
    operand: Operand,
}

impl Instruction {
    fn parse(l: usize, r: usize) -> Self {
        Self {
            opcode: InstructionOpCode::parse(l),
            operand: Operand::parse(r),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Value(usize),
    A,
    B,
    C,
}

impl Operand {
    fn parse(b: usize) -> Self {
        match b {
            0..=3 => Self::Value(b),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("got combo {b}"),
        }
    }

    fn resolve(self, registers: &Registers) -> usize {
        match self {
            Operand::Value(v) => v,
            Operand::A => registers.a,
            Operand::B => registers.b,
            Operand::C => registers.c,
        }
    }

    fn literal(self) -> usize {
        match self {
            Operand::Value(v) => v,
            Operand::A => 4,
            Operand::B => 5,
            Operand::C => 6,
        }
    }
}

fn eval(mut registers: Registers, mut program: Program) -> Vec<usize> {
    let mut out = vec![];

    while let Some(Instruction {
        opcode,
        operand: combo,
    }) = program.next_insn()
    {
        match opcode {
            InstructionOpCode::Adv => {
                let arg = combo.resolve(&registers);
                registers.a /= 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bxl => {
                let arg = combo.literal();
                registers.b ^= arg;
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bst => {
                let arg = combo.resolve(&registers);
                registers.b = arg % 8;
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Jnz => {
                if registers.a == 0 {
                    program.instruction_pointer += 2;
                } else {
                    let arg = combo.literal();
                    program.instruction_pointer = arg;
                }
            }
            InstructionOpCode::Bxc => {
                registers.b ^= registers.c;
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Out => {
                let arg = combo.resolve(&registers);
                out.push(arg % 8);
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bdv => {
                let arg = combo.resolve(&registers);
                registers.b = registers.a / 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Cdv => {
                let arg = combo.resolve(&registers);
                registers.c = registers.a / 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
        }
    }

    out
}

fn manual_sim(a: usize, out: &mut Vec<usize>) {
    // 2,4, 1,6, 7,5, 4,6, 1,4, 5,5, 0,3, 3,0

    // 2 4 = bst 4 = bst A
    // b = a % 8;

    // 1 6 = bxl 6  <- always as literal
    // b = (a % 8) ^ 6;

    // 7 5 = cdv 5 = cdv B
    // c = a / 2_usize.pow(((a % 8) ^ 6) as u32);

    // 4 6 = bxc 6 <- doesn't care about operand
    // b = ((a % 8) ^ 6) ^ (a / 2_usize.pow(((a % 8) ^ 6) as u32));

    // 1 4 = bxl 4 <- always as literal
    // let b2 = (((a % 8) ^ 6) ^ (a / 2_usize.pow(((a % 8) ^ 6) as u32))) ^ 4;

    // 5 5 = out 5 = out B
    out.push(print(a));

    // 0 3 = adv 3
    // let a2 = a / 8;

    // 3 0 = jnz 0
    if a / 8 == 0 {
        // NOOP, exit
    } else {
        // repeat
        manual_sim(a / 8, out);
    }
}

fn print(a: usize) -> usize {
    ((((a % 8) ^ 6) ^ (a / 2_usize.pow(((a % 8) ^ 6) as u32))) ^ 4) % 8
}

// To find X for given N so that print(X) = N
// use any number from `map.get(N)` + cycle * any
//
fn find_cycle() -> (usize, HashMap<usize, Vec<usize>>) {
    let mut seq = vec![];

    for i in 0..10_000 {
        seq.push(print(i));

        if seq.len() > 20 && seq.last_chunk::<20>() == seq.first_chunk::<20>() {
            // loop
            let full_cycle = &seq[0..seq.len() - 20];

            let length = full_cycle.len();
            let mut map = HashMap::<usize, Vec<usize>>::new();
            for (input, output) in full_cycle.iter().enumerate() {
                map.entry(*output).or_default().push(input);
            }
            for value in map.values_mut() {
                value.sort_unstable();
            }

            return (length, map);
        }
    }

    panic!("faled to find a cycle")
}
