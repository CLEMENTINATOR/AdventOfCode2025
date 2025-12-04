type Num = u32;

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|l| {
            let len = l.len();
            let mut max = 0;
            for i in 0..len {
                for j in (i + 1)..len {
                    let jolt = l.chars().nth(i).unwrap().to_digit(10).unwrap() * 10
                        + l.chars().nth(j).unwrap().to_digit(10).unwrap();
                    if jolt > max {
                        max = jolt;
                    }
                }
            }
            println!("{max}");
            max
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"987654321111111
811111111111119
234234234234278
818181911112111"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 357);
        assert_eq!(part2(EXAMPLE), 0);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 0);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
