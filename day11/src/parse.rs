use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{one_of, space1},
    combinator::{map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items_inspected: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub receiver_if_true: usize,
    pub receiver_if_false: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

pub fn parse_term<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Term, E> {
    alt((value(Term::Old, tag("old")), map(cc::u64, Term::Constant)))(i)
}

pub fn parse_operation<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    let (i, (l, op, r)) = preceded(
        tag("new = "),
        tuple((
            parse_term,
            preceded(space1, one_of("*+")),
            preceded(space1, parse_term),
        )),
    )(i)?;
    let op = match op {
        '*' => Operation::Mul(l, r),
        '+' => Operation::Add(l, r),
        _ => unreachable!(),
    };
    Ok((i, op))
}

pub fn parse_monkey<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Monkey, E> {
    let (i, _) = tuple((tag("Monkey "), cc::u64, tag(":"), tag("\r\n")))(i)?;
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cc::u64),
        tag("\r\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\r\n")))(i)?;
    let (i, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cc::u64, tag("\r\n")))(i)?;
    let (i, (_, _, receiver_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\r\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\r\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiver_if_true,
            receiver_if_false,
        },
    ))
}

pub fn parse_all_monkeys<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<Monkey>, E> {
    separated_list1(tag("\r\n"), parse_monkey)(i)
}