use std::ops::RangeInclusive;
use itertools::Itertools;

trait Swallows {
    fn contains_whole(&self, other: &Self) -> bool;
    fn contains_part(&self, other: &Self) -> bool;
}

impl<T> Swallows for RangeInclusive<T> 
where
    T: PartialOrd
{
    fn contains_whole(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn contains_part(&self, other: &Self) -> bool {
        (other.end() >= self.start() && other.end() <= self.end())
        || (other.start() >= self.start() && other.start() <= self.end())
    }
}

fn main() -> color_eyre::Result<()> {
    let answer1 = include_str!("../../data/day04.input")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| {
                    r.split('-')
                        .map(|b| b.parse::<u32>().expect("Range bound should be u32"))
                        .collect_tuple()
                        .map(|(begin, end)| begin..=end)
                        .expect("Could not construct range")
                })
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
                .expect("Each line must have 2 domains")
        })
        .filter(|(l, r)| l.contains_whole(r) || r.contains_whole(l))
        .count();
    println!("Answer for part 1 is {:?}", answer1);
    
    let answer2 = include_str!("../../data/day04.input")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| {
                    r.split('-')
                        .map(|b| b.parse::<u32>().expect("Range bound should be u32"))
                        .collect_tuple()
                        .map(|(begin, end)| begin..=end)
                        .expect("Could not construct range")
                })
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
                .expect("Each line must have 2 domains")
        })
        .filter(|(l, r)| l.contains_part(r) || r.contains_part(l))
        .count();
    println!("Answer for part 2 is {:?}", answer2);

    Ok(())
}
