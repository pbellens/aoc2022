use std::cmp::Reverse;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer1 = include_str!("../../data/day01.input")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(w)) = it.next() {
                sum = Some(sum.unwrap_or(0) + w);
            }
            sum
        })
        .max();
    println!("Answer for part 1 is {:?}", answer1);

    let answer2 = include_str!("../../data/day01.input")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(w)) = it.next() {
                sum = Some(sum.unwrap_or(0) + w);
            }
            sum
        })
        .map(Reverse)
        .k_smallest(3)
        .take(3)
        .map(|r| r.0)
        .sum::<u64>();
    println!("Answer for part 2 is {:?}", answer2);

    Ok(())
}
