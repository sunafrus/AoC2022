use std::{
    fmt,
    collections::{VecDeque, HashSet}
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value, all_consuming},
    sequence::preceded,
    IResult, Finish
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);

        alt((value(Self::Noop, noop), map(addx, Self::Addx)))(i)
    }

    fn cycles(self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

struct MachineState {
    instructions: VecDeque<Instruction>,
    current: Option<(Instruction, u32)>,
    cycle: u32,
    x: i32,
}

impl fmt::Debug for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cycle={} x={} current={:?} ({} instructions left)",
            self.cycle,
            self.x,
            self.current,
            self.instructions.len()
        )
    }
}

impl MachineState {
    fn new() -> Self {
        let mut res = Self {
            instructions: include_str!("input.txt")
                .lines()
                .map(|line| all_consuming(Instruction::parse)(line).finish().unwrap().1)
                .collect(),
            current: None,
            cycle: 1,
            x: 1,
        };

        res.decode();

        res
    }

    fn decode(&mut self) {
        self.current = self.instructions.pop_front().map(|ins| (ins, ins.cycles()));
    }

    fn step(&mut self) -> bool {
        if self.current.is_none() {
            return false;
        }

        let (ins, cycles_left) = self.current.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            match ins {
                Instruction::Noop => {},
                Instruction::Addx(x) => self.x += *x,
            }
            self.decode();
        }
        self.cycle += 1;

        true
    }
}

fn main() {
    let mut ms = MachineState::new();

    let count_cycles = [20,60,100,140,180,220]
        .into_iter()
        .collect::<HashSet<u32>>();
    let mut sum: i32 = 0;
    
    loop {
        println!("{ms:?}");

        if count_cycles.contains(&ms.cycle) {
            sum += ms.cycle as i32 * ms.x;
        }

        if !ms.step() {
            break;
        }
    }

    dbg!(sum);
}
