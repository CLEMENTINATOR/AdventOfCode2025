type Num = u32;

#[derive(Debug)]
enum Direction {
    Left(u32),
    Right(u32),
}

pub fn part1(input: &str) -> Num {
    let mut dial_position: u32 = 50;
    input
        .lines()
        .map(|l| {
            let (dir, count) = l.split_at(1);
            let count: u32 = count.parse().unwrap();
            match dir {
                "L" => Direction::Left(count % 100),
                "R" => Direction::Right(count % 100),
                _ => panic!("invalid dir {dir}"),
            }
        })
        .map(|dir| {
            dial_position = match dir {
                Direction::Left(count) => match dial_position.checked_sub(count) {
                    Some(res) => res,
                    None => 100 - (count - dial_position),
                },

                Direction::Right(count) => (dial_position + count) % 100,
            };
            dial_position
        })
        .filter(|i| *i == 0)
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> Num {
    let mut dial_position: u32 = 50;
    input
        .lines()
        .map(|l| {
            let (dir, count) = l.split_at(1);
            let count: u32 = count.parse().unwrap();
            match dir {
                "L" => Direction::Left(count),
                "R" => Direction::Right(count),
                _ => panic!("invalid dir {dir}"),
            }
        })
        .map(|dir| {
            let (res, passed_zero_count) = match dir {
                Direction::Left(count) => {
                    let steps = count % 100;
                    if let Some(dial_position) = dial_position.checked_sub(steps) {
                        let passed_zero = if dial_position == 0 { 1 } else { 0 };
                        (dial_position, count / 100 + passed_zero)
                    } else {
                        let passed_zero = if dial_position == 0 { 0 } else { 1 };
                        (100 - (steps - dial_position), count / 100 + passed_zero)
                    }
                }

                Direction::Right(count) => {
                    ((dial_position + count) % 100, (dial_position + count) / 100)
                }
            };
            dial_position = res;
            passed_zero_count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 3);
        assert_eq!(part2(EXAMPLE), 6);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 1040);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 6027);
    }
}
