use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use Direction::*;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Direction> {
    let re = Regex::new(r"^([a-z]+) (\d+)$").unwrap();
    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();
        let amount: isize = caps[2].parse().unwrap();
        match &caps[1] {
            "forward" => Forward(amount),
            "down" => Down(amount),
            _ => Up(amount),
        }
    }).collect()
}

#[aoc(day2, part1)]
fn part1(commands: &Vec<Direction>) -> usize {
    let mut depth = 0;
    let mut horpos = 0;
    for command in commands {
        match command {
            Forward(v) => horpos += v,
            Down(v) => depth += v,
            Up(v) => depth -= v,
        }
    };
    depth as usize * horpos as usize
}

#[aoc(day2, part2)]
fn part2(commands: &Vec<Direction>) -> usize {
    let mut aim = 0;
    let mut depth = 0;
    let mut horpos = 0;

    for command in commands {
        match command {
            Forward(v) => {
                horpos += v;
                depth += v * aim;
            },
            Down(v) => aim += v,
            Up(v) => aim -= v,
        }
    };
    depth as usize * horpos as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(
            150,
            part1(&input),
        );
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(
            900,
            part2(&input),
        );
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
           forward 5
           down 5
           forward 8
           up 3
           down 8
           forward 2
        "}
    }
}
