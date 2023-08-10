use itertools::Itertools;
use std::ops::RangeInclusive;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let completely_redundant = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("Range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("Each range should have a start and an end")
                })
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
                .expect("Each line must have a pair of ranges")
        })
        .filter(|(a, b)| a.completely_overlaps(b))
        .count();

    dbg!(completely_redundant);
}

fn part_two() {
    let partially_redundant = include_str!("input.txt")
    .lines()
    .map(|line| {
        line.split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|n| n.parse().expect("Range start/end should be u32"))
                    .collect_tuple::<(u32, u32)>()
                    .map(|(start, end)| start..=end)
                    .expect("Each range should have a start and an end")
            })
            .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
            .expect("Each line must have a pair of ranges")
    })
    .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
    .count();

    
    dbg!(partially_redundant);
}

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn completely_overlaps(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}
