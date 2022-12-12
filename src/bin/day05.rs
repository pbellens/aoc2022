use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    Finish, IResult,
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Crate(char);

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    map(
        nom::sequence::delimited(tag("["), take(1_usize), tag("]")),
        |id: &str| Crate(id.chars().next().unwrap()))(i)
}

fn parse_empty(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_nothing(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_empty, |_| None)))(i)
}

fn parse_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_nothing(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(nom::sequence::preceded(tag(" "), parse_crate_or_nothing))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    nom::combinator::map_res(
        nom::bytes::complete::take_while1(|c: char| c.is_ascii_digit()), 
        |s: &str| {s.parse::<usize>()
    })(i)
}
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    nom::combinator::map(
        parse_number,
        |i| i-1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}
fn parse_cmd(i: &str) -> IResult<&str, Instruction> {
    map(
        nom::sequence::tuple((
            nom::sequence::preceded(tag("move "), parse_number),
            nom::sequence::preceded(tag(" from "), parse_pile_number),
            nom::sequence::preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn schmuffle(self: &mut Self, i: &Instruction) {
        for _ in 0..i.quantity {
            let c = self.0[i.src].pop().unwrap();
            self.0[i.dst].push(c);
        }
    }
}

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() -> color_eyre::Result<()> {
    let mut lines = include_str!("../../data/day05.input").lines();

    let cratelines = (&mut lines)
        .map_while(|l| {
            all_consuming(parse_line)(l)
                .finish()
                .ok()
                .map(|(_, parsedline)| parsedline)
        })
        .collect::<Vec<_>>();

    let mut piles = Piles(transpose(cratelines));
    //println!("{piles:?}");

    assert!(lines.next().unwrap().is_empty());

    for i in lines
        .map(|line| all_consuming(parse_cmd)(line).finish().unwrap().1) 
    {
        piles.schmuffle(&i);
    }

    let answer1 = piles.0
        .iter()
        .map(|p| p.last().unwrap()) 
        .collect::<Vec<_>>();
    println!("Answer for part 1 is {:?}", answer1);

    Ok(())
}
