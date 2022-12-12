use std::convert::TryFrom;
use im::HashSet;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        return match v {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(v)),
            _ => Err(color_eyre::eyre::eyre!("Item {v:?} does not fit in the rucksack"))
        }
    }
}

impl Item {
    fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer1 = include_str!("../../data/day03.input")
        .lines()
        .map(|l| {
            let (r1, r2) = l.split_at(l.len() / 2);
            let items1 = r1.bytes()
                .map(|i| Item::try_from(i))
                .collect::<Result<HashSet<Item>, _>>()?;
            let items2 = r2.bytes()
                .map(|i| Item::try_from(i))
                .collect::<Result<HashSet<_>, _>>()?;
            let dupls = items1.intersection(items2);
            dupls.iter().next().map(|d| d.score()).ok_or(color_eyre::eyre::eyre!("No duplicates in rucksack"))
        })
        .sum::<color_eyre::Result<usize>>();
    println!("Answer for part 1 is {:?}", answer1);

    let answer2: usize = include_str!("../../data/day03.input")
        .lines()
        .map(|l| {
            l.bytes()
            .map(|b| b.try_into().unwrap())
            .collect::<HashSet<Item>>()
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            chunks
                .reduce(|a, b| a.intersection(b))
                .expect("We have 3 rucksacks, this should be ok")
                .iter()
                .next()
                .expect("There must be at least 1 common item")
                .score()
        })
        .sum();
    println!("Answer for part 2 is {:?}", answer2);

    Ok(())
}
