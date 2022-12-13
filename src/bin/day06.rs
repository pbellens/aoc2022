use std::collections::HashSet;

fn find_start_marker(input: &str, window: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == window)
        .map(|pos| pos + window)
}

fn main() -> color_eyre::Result<()> {
    let answer1 = find_start_marker(include_str!("../../data/day06.input"), 4);
    println!("Answer for part 1 is {:?}", answer1);

    let answer2 = find_start_marker(include_str!("../../data/day06.input"), 14);
    println!("Answer for part 2 is {:?}", answer2);

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::find_start_marker;

    #[test]
    fn test_find_start_marker() {
        assert_eq!(Some(7), find_start_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4))
    }
}
