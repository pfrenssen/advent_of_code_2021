use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day3, part1)]
fn part1(report: &[Vec<char>]) -> usize {
    let mcb: String = transpose(report.to_owned())
        .iter()
        .map(|vc| -> String {
            match vc.iter().filter(|&c| c.eq(&'0')).count() > vc.len() / 2 {
                true => "0",
                false => "1",
            }
            .to_string()
        })
        .collect();
    let intval = usize::from_str_radix(mcb.as_str(), 2).unwrap();
    let bitmask = 2usize.pow(mcb.len() as u32) - 1;
    intval * (bitmask & !intval)
}

#[aoc(day3, part2)]
fn part2(report: &[Vec<char>]) -> usize {
    let numsize = report[0].len();

    let mut oxygen = report.to_owned();
    for i in 0..numsize {
        if oxygen.len() == 1 {
            break;
        };
        let num0 = transpose(oxygen.clone())[i]
            .iter()
            .filter(|&c| c.eq(&'0'))
            .count();
        let num1 = oxygen.len() - num0;

        oxygen = match num0.cmp(&num1) {
            Ordering::Less => filter(oxygen, '1', i),
            Ordering::Greater => filter(oxygen, '0', i),
            Ordering::Equal => filter(oxygen, '1', i),
        };
    }
    let oxygen = oxygen[0].iter().collect::<String>();
    let oxygen = usize::from_str_radix(oxygen.as_str(), 2).unwrap();

    let mut co2 = report.to_owned();
    for i in 0..numsize {
        if co2.len() == 1 {
            break;
        };
        let num0 = transpose(co2.clone())[i]
            .iter()
            .filter(|&c| c.eq(&'0'))
            .count();
        let num1 = co2.len() - num0;

        co2 = match num0.cmp(&num1) {
            Ordering::Less => filter(co2, '0', i),
            Ordering::Greater => filter(co2, '1', i),
            Ordering::Equal => filter(co2, '0', i),
        };
    }
    let co2 = co2[0].iter().collect::<String>();
    let co2 = usize::from_str_radix(co2.as_str(), 2).unwrap();

    oxygen * co2
}

// https://stackoverflow.com/a/64499219/350644
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn filter(col: Vec<Vec<char>>, char: char, pos: usize) -> Vec<Vec<char>> {
    col.into_iter().filter(|x| x[pos].eq(&char)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<Vec<char>> = vec![
            String::from("00100").chars().collect(),
            String::from("11110").chars().collect(),
            String::from("10110").chars().collect(),
            String::from("10111").chars().collect(),
            String::from("10101").chars().collect(),
            String::from("01111").chars().collect(),
            String::from("00111").chars().collect(),
            String::from("11100").chars().collect(),
            String::from("10000").chars().collect(),
            String::from("11001").chars().collect(),
            String::from("00010").chars().collect(),
            String::from("01010").chars().collect(),
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(198, part1(&input),);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(230, part2(&input),);
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        "}
    }
}
