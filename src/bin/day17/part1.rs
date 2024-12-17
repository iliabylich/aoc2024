fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

impl Registers {
    fn parse(input: &str) -> Self {
        let (a, rest) = input.split_once("\n").unwrap();
        let (b, c) = rest.split_once("\n").unwrap();

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

#[derive(Debug)]
struct Program {
    instruction_pointer: usize,
    tape: Vec<usize>,
}

impl Program {
    fn parse(input: &str) -> Self {
        let tape = input.trim().strip_prefix("Program: ").unwrap();
        let tape = tape
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            instruction_pointer: 0,
            tape,
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

fn solve(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = Registers::parse(registers);
    let mut program = Program::parse(program);

    let mut out = vec![];

    while let Some(Instruction {
        opcode,
        operand: combo,
    }) = program.next_insn()
    {
        // println!("Running {:?} {:?}", opcode, combo);

        match opcode {
            InstructionOpCode::Adv => {
                let arg = combo.resolve(&mut registers);
                registers.a = registers.a / 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bxl => {
                let arg = combo.literal();
                registers.b = registers.b ^ arg;
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bst => {
                let arg = combo.resolve(&mut registers);
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
                registers.b = registers.b ^ registers.c;
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Out => {
                let arg = combo.resolve(&mut registers);
                out.push(format!("{}", arg % 8));
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Bdv => {
                let arg = combo.resolve(&mut registers);
                registers.b = registers.a / 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
            InstructionOpCode::Cdv => {
                let arg = combo.resolve(&mut registers);
                registers.c = registers.a / 2_usize.pow(arg as u32);
                program.instruction_pointer += 2;
            }
        }
    }

    let out = out.join(",");

    out
}

#[test]
fn test1() {
    let input = include_str!("input_test1.txt");
    let output = solve(input);
    assert_eq!(output, "4,6,3,5,6,3,5,2,1,0".to_string());
}
