use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(measurements: &[i32]) -> usize {
    measurements.windows(2).filter(|w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
fn part2(measurements: &[i32]) -> usize {
    let moving_average: Vec<i32> = measurements
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect();
    part1(&moving_average)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
