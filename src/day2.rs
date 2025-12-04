type Num = u64;

pub fn part1(input: &str) -> Num {
    let init: u64 = 0;
    input
        .trim_end()
        .split(",")
        .fold(init, |acc, product_range| {
            let idx = product_range.find("-").unwrap();
            let start: u64 = product_range[0..idx].parse().unwrap();
            let end: u64 = product_range[(idx + 1)..].parse().unwrap();

            let mut acc = acc;

            for i in start..(end + 1) {
                let i_str = i.to_string();
                let len = i_str.len();
                if len % 2 != 0 {
                    continue;
                }
                let a = &i_str[0..(len / 2)];
                let b = &i_str[(len / 2)..];
                if a == b {
                    acc += i;
                }
            }
            acc
        })
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 1227775554);
        assert_eq!(part2(EXAMPLE), 0);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 23560874270);
    }

    //#[test]
    //fn run_part2() {
    //    let input = crate::utils::get_day_input!();
    //    let output = part2(&input);
    //    println!("Part 2: {}", output);
    //    assert_eq!(output, 0);
    //}
}
