fn main() {
    let input = include_str!("input.txt");
    let output = solve(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    px: u64,
    py: u64,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let (line_a, input) = input.split_once('\n').unwrap();
        let (line_b, line_prize) = input.split_once('\n').unwrap();

        let line_a = line_a.strip_prefix("Button A: ").unwrap();
        let line_b = line_b.strip_prefix("Button B: ").unwrap();
        let line_prize = line_prize.strip_prefix("Prize: ").unwrap();

        let (ax, ay) = line_a.split_once(", ").unwrap();
        let (bx, by) = line_b.split_once(", ").unwrap();
        let (px, py) = line_prize.split_once(", ").unwrap();

        let ax = ax.strip_prefix("X+").unwrap();
        let ay = ay.strip_prefix("Y+").unwrap();
        let bx = bx.strip_prefix("X+").unwrap();
        let by = by.strip_prefix("Y+").unwrap();
        let px = px.strip_prefix("X=").unwrap();
        let py = py.strip_prefix("Y=").unwrap();

        Self {
            ax: ax.parse().unwrap(),
            ay: ay.parse().unwrap(),
            bx: bx.parse().unwrap(),
            by: by.parse().unwrap(),
            px: px.parse::<u64>().unwrap(), // + 10000000000000,
            py: py.parse::<u64>().unwrap(), // + 10000000000000,
        }
    }

    fn moves(self) -> Option<(u64, u64)> {
        fn gcd(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }
        fn lcm(a: u64, b: u64) -> u64 {
            a.checked_div(gcd(a, b)).unwrap().checked_mul(b).unwrap()
        }

        let Self {
            mut ax,
            ay,
            mut bx,
            mut by,
            mut px,
            mut py,
        } = self;

        let lcm = lcm(ax, ay);

        let mulx = lcm.checked_div(ax).unwrap();
        let muly = lcm.checked_div(ay).unwrap();

        ax = lcm;
        bx = bx.checked_mul(mulx).unwrap();
        px = px.checked_mul(mulx).unwrap();

        // ay = lcm;
        by = by.checked_mul(muly).unwrap();
        py = py.checked_mul(muly).unwrap();

        if py < px && by < bx {
            std::mem::swap(&mut py, &mut px);
            std::mem::swap(&mut by, &mut bx);
        }

        let lhs = py.checked_sub(px)?;
        let rhs = by.checked_sub(bx)?;
        if lhs % rhs != 0 {
            return None;
        }
        let nb = lhs.checked_div(rhs).unwrap();

        let lhs = px.checked_sub(nb.checked_mul(bx).unwrap())?;
        let rhs = ax;
        if lhs % rhs != 0 {
            return None;
        }
        let na = lhs.checked_div(rhs).unwrap();

        Some((na, nb))
    }

    fn price(self) -> Option<u64> {
        let (na, nb) = self.moves()?;
        Some(na * 3 + nb)
    }
}

fn solve(input: &str) -> u64 {
    let mut machines = vec![];
    for subinput in input.trim().split("\n\n") {
        machines.push(Machine::parse(subinput));
    }

    machines.into_iter().filter_map(|m| m.price()).sum()
}

#[test]
fn test() {
    let input = include_str!("input_test.txt");
    let output = solve(input);
    assert_eq!(output, 480);
}
