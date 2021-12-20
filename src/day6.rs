use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> [u64; 9] {
    let fishes: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut school = [0_u64; 9];
    for fish in fishes {
        school[fish] += 1;
    }
    school
}

#[aoc(day6, part1)]
fn part1(school: &[u64; 9]) -> u64 {
    count_school(school, 80)
}

#[aoc(day6, part2)]
fn part2(school: &[u64; 9]) -> u64 {
    count_school(school, 256)
}

fn count_school(school: &[u64; 9], days: usize) -> u64 {
    let mut school = *school;
    for _ in 0..days {
        let due = school[0];
        for i in 0..8 {
            school[i] = school[i + 1];
        }
        school[6] += due;
        school[8] = due;
    }
    school.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: [u64; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(5934, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(26984457539, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            3,4,3,1,2
        "}
    }
}
