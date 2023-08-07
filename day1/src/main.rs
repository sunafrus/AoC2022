use itertools::Itertools;

fn main() {
    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    println!("{max:?}");
}
