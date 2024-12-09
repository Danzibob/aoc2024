#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

type V = (isize,isize);


impl Grid {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Grid::new(data)
    }

    pub fn get(&self, p: V) -> Option<char> {
        let x: usize = p.0.try_into().ok()?;
        let y: usize = p.1.try_into().ok()?;

        if x < self.width && y < self.height {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    pub fn match_string(&self, start: V, d: V, word: &[char]) -> bool {
        let (mut posx, mut posy) = start;
        for c in word{
            if let Some(l) = self.get((posx, posy)){
                if l == *c {
                    posx += d.0;
                    posy += d.1;
                    continue;
                }
            }
            // otherwise
            return false
        }
        return true;
    }

    pub fn get_x(&self, start: V) -> Option<(char,char,char,char)> {
        let tl = self.get((start.0-1, start.1-1))?;
        let bl = self.get((start.0-1, start.1+1))?;
        let tr = self.get((start.0+1, start.1-1))?;
        let br = self.get((start.0+1, start.1+1))?;
        return Some((tl, tr, bl, br))
    }

}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    Grid::from_string(input)
}

const DIRS: [V; 8] = [
    (0,1),
    (1,1),
    (1,0),
    (1,-1),
    (0,-1),
    (-1,-1),
    (-1,0),
    (-1,1)
];

#[aoc(day4, part1)]
pub fn solve_part1(input: &Grid) -> usize {
    const XMAS:[char;4] = ['X','M','A','S'];
    let mut count = 0;
    for x in 0..(input.width){
        for y in 0..(input.height){
            for d in DIRS {
                if input.match_string((x as isize,y as isize), d, &XMAS){
                    count += 1
                }
            }
        }
    }
    count
}

fn is_mas(letters: (char, char)) -> bool{
    match letters{
        ('M','S') | ('S','M') => true,
        _ => false
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Grid) -> usize {
    let mut count = 0;
    for x in 0..(input.width){
        for y in 0..(input.height){
            let pos = (x as isize,y as isize);

            // Continue if center is invalid or is not A
            if let Some(center) = input.get(pos){
                if center != 'A'{continue}
            } else {continue}

            if let Some(cross) = input.get_x(pos){
                let fwd = (cross.0,cross.3);
                let bwd = (cross.1,cross.2);
                if is_mas(fwd) && is_mas(bwd){
                    count += 1;
                }

            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input/2024/day4.txt");
    const MINI_INPUT: &str = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";

    #[test]
    fn test_mini() {
        let grid = input_generator(MINI_INPUT);
        assert_eq!(solve_part1(&grid), 4);
    }

    #[test]
    fn test_part1() {
        let grid = input_generator(TEST_INPUT);
        assert_eq!(solve_part1(&grid), 18);
    }

    #[test]
    fn test_part2() {
        let grid = input_generator(TEST_INPUT);
        assert_eq!(solve_part2(&grid), 9);
    }
}
