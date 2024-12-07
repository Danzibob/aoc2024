use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mul = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    input
        .lines()
        .map(|line| {
            mul.captures_iter(line).map(|caps| {
                let a: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let b: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                (a, b)
            })
        })
        .flatten()
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let instruction = Regex::new(r"(mul)\(([0-9]+)\,([0-9]+)\)|(do)\(\)|(don\'t)\(\)").unwrap();
    let mut acc = 0;
    let mut enable = true;
    input
        .lines()
        .for_each(|line| {
            for i in instruction.captures_iter(line){
                // Matches a mul(_,_) instruction
                if i.get(1).is_some() && enable {
                    let a:usize = i.get(2).unwrap().as_str().parse().unwrap();
                    let b:usize = i.get(3).unwrap().as_str().parse().unwrap();
                    acc += a*b
                } else 
                // Matches a do() instruction
                if i.get(4).is_some() {
                    enable = true;
                } else 
                // Matches a don't() instruction
                if i.get(5).is_some() {
                    enable = false
                }
            }
        });
    return acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input/2024/day3.txt");
    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 161)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 48)
    }
}
