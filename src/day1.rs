use std::iter::zip;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (mut a, mut b): (Vec<isize>, Vec<isize>) = input
    .lines()
    .map(|l| {
        let mut nums = l.split_ascii_whitespace().map(|n|
            n.parse::<isize>().expect("Failed to parse an int")
        );
        (nums.next().unwrap(), nums.next().unwrap())
    }).unzip();

    a.sort_unstable();
    b.sort_unstable();
    zip(a,b).map(|(a,b)| (a-b).abs()).sum::<isize>() as usize
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (a, mut b): (Vec<usize>, Vec<usize>) = input
    .lines()
    .map(|l| {
        let mut nums = l.split_ascii_whitespace().map(|n|
            n.parse::<usize>().expect("Failed to parse an int")
        );
        (nums.next().unwrap(), nums.next().unwrap())
    }).unzip();

    b.sort_unstable();

    a.iter().map(|x| 
        x * b.iter().filter(|&y| y==x).count()
    ).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT:&str = include_str!("../test_input/2024/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 31);
    }
}