use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::min;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(positions: &[usize]) -> usize {
    let mut positions = positions.to_owned();
    positions.sort_unstable();
    let target = positions[positions.len() / 2] as isize;
    let mut fuel: isize = 0;
    for position in positions {
        let distance = position as isize - target;
        fuel += distance.abs();
    }
    fuel as usize
}

#[aoc(day7, part2)]
fn part2(positions: &[usize]) -> usize {
    let mut positions = positions.to_owned();
    positions.sort_unstable();
    let avg = |p: &[usize]| -> f32 { p.iter().map(|x| *x as f32).sum::<f32>() / p.len() as f32 };
    let fuel_cost = |p: &[usize], t: usize| -> usize {
        p.iter()
            .map(|x| {
                let d = if x > &t { x - t } else { t - x };
                d * (d + 1) / 2
            })
            .sum()
    };
    min(
        fuel_cost(&positions, avg(&positions).ceil() as usize),
        fuel_cost(&positions, avg(&positions).floor() as usize),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<usize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(37, part1(&input),);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(168, part2(&input),);
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            16,1,2,0,4,2,7,1,2,14
        "}
    }
}
