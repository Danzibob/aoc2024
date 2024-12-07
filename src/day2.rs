#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum STATE {
    RISING,
    FALLING,
    UNSAFE,
}

fn get_state(delta: isize) -> STATE {
    match delta {
        1 | 2 | 3 => STATE::RISING,
        -1 | -2 | -3 => STATE::FALLING,
        _ => STATE::UNSAFE,
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<isize>().expect("Failed to parse int"))
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .map(|rec| {
            let mut record = rec.iter();
            let mut last = record.next().unwrap();
            let mut state: Option<STATE> = None;
            for next in record {
                let new_state = get_state(next - last);
                if new_state == STATE::UNSAFE {
                    return 0;
                }
                // If we already know the direction and it doesn't match, unsafe!
                if let Some(old_state) = state {
                    if old_state != new_state {
                        return 0;
                    }
                } else {
                    state = Some(new_state);
                }
                last = next;
            }
            return 1;
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .map(|record| {
            if solve_part1(&[record.clone()]) > 0 {
                return 1;
            } else {
                // make the other lists & test those
                for i in 0..record.len() {
                    let mut x = record.clone();
                    x.remove(i);
                    if solve_part1(&[x]) > 0 {
                        return 1;
                    }
                }
            };
            // If none of that works, it won't work
            return 0;
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input/2024/day2.txt");

    #[test]
    fn test_part1() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(solve_part1(&input), 2)
    }

    #[test]
    fn test_part2() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(solve_part2(&input), 6);
    }
}
