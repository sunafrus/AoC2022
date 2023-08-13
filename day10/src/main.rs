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
    display_lines: Vec<u64>,
}

impl fmt::Debug for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "cycle={} x={} current={:?} ({} instructions left)",
            self.cycle,
            self.x,
            self.current,
            self.instructions.len()
        )?;

        for line in &self.display_lines {
            for i in 0..40 {
                let c = if line & cycle_mask(i) > 0 { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MachineState {
    fn new() -> Self {
        let mut res = Self {
            instructions: include_str!("input.txt")
                .lines()
                .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
                .collect(),
            current: None,
            cycle: 0,
            x: 1,
            display_lines: vec![],
        };
        res.decode();
        res
    }

    fn decode(&mut self) {
        self.current = self.instructions.pop_front().map(|ins| (ins, ins.cycles()));
    }

    fn draw(&mut self) {
        let crt_line = (self.cycle / 40) as usize;
        if crt_line + 1 > self.display_lines.len() {
            self.display_lines.push(0);
        }
        let crt_line = self.display_lines.get_mut(crt_line).unwrap();
        let cycle_mask = cycle_mask(self.cycle);
        let sprite = sprite_value(self.x as _);
        *crt_line |= cycle_mask & sprite;
    }

    fn step(&mut self) -> bool {
        if self.current.is_none() {
            return false;
        }

        let (ins, cycles_left) = self.current.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            match ins {
                Instruction::Noop => {}
                Instruction::Addx(x) => self.x += *x,
            }
            self.decode();
        }

        self.cycle += 1;
        true
    }
}


const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;

fn sprite_value(pos: i32) -> u64 {
    let model = 0b11100000000000000000000000000000000000000_u64;
    let shifted;
    if pos < 0 {
        (shifted, _) = model.overflowing_shl((-pos).try_into().unwrap());
    } else {
        (shifted, _) = model.overflowing_shr(pos.try_into().unwrap());
    }
    shifted & DISPLAY_MASK
}

fn cycle_mask(cycle: u32) -> u64 {
    (0b1000000000000000000000000000000000000000 >> (cycle % 40)) & DISPLAY_MASK
}
#[test]
fn test_sprite_value() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        format!("{:040b}", sprite_value(0)),
        "1100000000000000000000000000000000000000"
    );
    assert_eq!(
        format!("{:040b}", sprite_value(1)),
        "1110000000000000000000000000000000000000"
    );
    assert_eq!(
        format!("{:040b}", sprite_value(38)),
        "0000000000000000000000000000000000000111"
    );
    assert_eq!(
        format!("{:040b}", sprite_value(39)),
        "0000000000000000000000000000000000000011"
    );
    assert_eq!(
        format!("{:040b}", sprite_value(40)),
        "0000000000000000000000000000000000000001"
    );
}

fn main() {
    let mut ms = MachineState::new();

    let count_cycles = [20,60,100,140,180,220]
        .into_iter()
        .collect::<HashSet<u32>>();
    let mut sum: i32 = 0;
    
    loop {
        ms.draw();
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
