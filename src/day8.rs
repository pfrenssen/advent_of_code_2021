use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::string::ToString;
use std::vec;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" | ").collect::<Vec<&str>>();
            (
                parts[0]
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect(),
                parts[1]
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(notes: &[(Vec<String>, Vec<String>)]) -> usize {
    const FIXED_WIDTH_DIGIT_LENGTHS: [usize; 4] = [2, 3, 4, 7];
    notes
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .filter(|&d| FIXED_WIDTH_DIGIT_LENGTHS.contains(&d.len()))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(notes: &[(Vec<String>, Vec<String>)]) -> usize {
    let superset = |set: &Vec<String>, search: &String| -> String {
        set.iter()
            .fold(None, |acc, p| {
                if let Some(q) = acc {
                    Some(q)
                } else {
                    for c in search.chars() {
                        if !p.contains(c) {
                            return None;
                        }
                    }
                    Some(p)
                }
            })
            .unwrap()
            .to_string()
    };

    let not_superset = |set: &Vec<String>, search: &String| -> String {
        set.iter()
            .fold(None, |acc, p| {
                if let Some(q) = acc {
                    Some(q)
                } else {
                    for c in search.chars() {
                        if !p.contains(c) {
                            return Some(p);
                        }
                    }
                    None
                }
            })
            .unwrap()
            .to_string()
    };

    let subset = |set: &Vec<String>, search: &String| -> String {
        set.iter()
            .fold(None, |acc, p| {
                if let Some(q) = acc {
                    Some(q)
                } else {
                    for c in p.chars() {
                        if !search.contains(c) {
                            return None;
                        }
                    }
                    Some(p)
                }
            })
            .unwrap()
            .to_string()
    };

    let sortstrings = |set: Vec<String>| -> Vec<String> {
        set.iter()
            .map(|p| p.chars().sorted().collect::<String>())
            .collect()
    };

    let mut total = 0;

    for note in notes {
        let (patterns, value) = note;
        let patterns = sortstrings(patterns.clone());
        let value = sortstrings(value.clone());

        let mut digits = vec![String::from(""); 10];

        // Sort the patterns by number of segments.
        let mut s5 = vec![];
        let mut s6 = vec![];
        for pattern in patterns {
            match pattern.len() {
                2 => digits[1] = pattern.to_string(),
                3 => digits[7] = pattern.to_string(),
                4 => digits[4] = pattern.to_string(),
                5 => s5.push(pattern.to_string()),
                6 => s6.push(pattern.to_string()),
                7 => digits[8] = pattern.to_string(),
                _ => unreachable!(),
            }
        }

        // "3" has 5 segments and contains all the segments of "7".
        digits[3] = superset(&s5, &digits[7]);
        s5.remove(s5.iter().position(|p| *p == digits[3]).unwrap());

        // "6" has 6 segments but misses a segment of "7".
        digits[6] = not_superset(&s6, &digits[7]);
        s6.remove(s6.iter().position(|p| *p == digits[6]).unwrap());

        // "9" has 6 segments and is a superset of "4".
        digits[9] = superset(&s6, &digits[4]);
        s6.remove(s6.iter().position(|p| *p == digits[9]).unwrap());

        // The remaining 6 segment digit is "0".
        digits[0] = s6[0].clone();

        // "5" is a subset of "9".
        digits[5] = subset(&s5, &digits[9]);
        s5.remove(s5.iter().position(|p| *p == digits[5]).unwrap());

        // The remaining 5 segment digit is "2".
        digits[2] = s5[0].clone();

        let values: Vec<usize> = value
            .iter()
            .map(|d| digits.iter().position(|x| x == d).unwrap())
            .collect();
        let value: usize = format!("{}{}{}{}", values[0], values[1], values[2], values[3])
            .parse()
            .unwrap();
        total += value;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<(Vec<String>, Vec<String>)> = vec![
            (
                vec![
                    String::from("be"),
                    String::from("cfbegad"),
                    String::from("cbdgef"),
                    String::from("fgaecd"),
                    String::from("cgeb"),
                    String::from("fdcge"),
                    String::from("agebfd"),
                    String::from("fecdb"),
                    String::from("fabcd"),
                    String::from("edb"),
                ],
                vec![
                    String::from("fdgacbe"),
                    String::from("cefdb"),
                    String::from("cefbgd"),
                    String::from("gcbe"),
                ],
            ),
            (
                vec![
                    String::from("edbfga"),
                    String::from("begcd"),
                    String::from("cbg"),
                    String::from("gc"),
                    String::from("gcadebf"),
                    String::from("fbgde"),
                    String::from("acbgfd"),
                    String::from("abcde"),
                    String::from("gfcbed"),
                    String::from("gfec"),
                ],
                vec![
                    String::from("fcgedb"),
                    String::from("cgb"),
                    String::from("dgebacf"),
                    String::from("gc"),
                ],
            ),
            (
                vec![
                    String::from("fgaebd"),
                    String::from("cg"),
                    String::from("bdaec"),
                    String::from("gdafb"),
                    String::from("agbcfd"),
                    String::from("gdcbef"),
                    String::from("bgcad"),
                    String::from("gfac"),
                    String::from("gcb"),
                    String::from("cdgabef"),
                ],
                vec![
                    String::from("cg"),
                    String::from("cg"),
                    String::from("fdcagb"),
                    String::from("cbg"),
                ],
            ),
            (
                vec![
                    String::from("fbegcd"),
                    String::from("cbd"),
                    String::from("adcefb"),
                    String::from("dageb"),
                    String::from("afcb"),
                    String::from("bc"),
                    String::from("aefdc"),
                    String::from("ecdab"),
                    String::from("fgdeca"),
                    String::from("fcdbega"),
                ],
                vec![
                    String::from("efabcd"),
                    String::from("cedba"),
                    String::from("gadfec"),
                    String::from("cb"),
                ],
            ),
            (
                vec![
                    String::from("aecbfdg"),
                    String::from("fbg"),
                    String::from("gf"),
                    String::from("bafeg"),
                    String::from("dbefa"),
                    String::from("fcge"),
                    String::from("gcbea"),
                    String::from("fcaegb"),
                    String::from("dgceab"),
                    String::from("fcbdga"),
                ],
                vec![
                    String::from("gecf"),
                    String::from("egdcabf"),
                    String::from("bgf"),
                    String::from("bfgea"),
                ],
            ),
            (
                vec![
                    String::from("fgeab"),
                    String::from("ca"),
                    String::from("afcebg"),
                    String::from("bdacfeg"),
                    String::from("cfaedg"),
                    String::from("gcfdb"),
                    String::from("baec"),
                    String::from("bfadeg"),
                    String::from("bafgc"),
                    String::from("acf"),
                ],
                vec![
                    String::from("gebdcfa"),
                    String::from("ecba"),
                    String::from("ca"),
                    String::from("fadegcb"),
                ],
            ),
            (
                vec![
                    String::from("dbcfg"),
                    String::from("fgd"),
                    String::from("bdegcaf"),
                    String::from("fgec"),
                    String::from("aegbdf"),
                    String::from("ecdfab"),
                    String::from("fbedc"),
                    String::from("dacgb"),
                    String::from("gdcebf"),
                    String::from("gf"),
                ],
                vec![
                    String::from("cefg"),
                    String::from("dcbef"),
                    String::from("fcge"),
                    String::from("gbcadfe"),
                ],
            ),
            (
                vec![
                    String::from("bdfegc"),
                    String::from("cbegaf"),
                    String::from("gecbf"),
                    String::from("dfcage"),
                    String::from("bdacg"),
                    String::from("ed"),
                    String::from("bedf"),
                    String::from("ced"),
                    String::from("adcbefg"),
                    String::from("gebcd"),
                ],
                vec![
                    String::from("ed"),
                    String::from("bcgafe"),
                    String::from("cdgba"),
                    String::from("cbgef"),
                ],
            ),
            (
                vec![
                    String::from("egadfb"),
                    String::from("cdbfeg"),
                    String::from("cegd"),
                    String::from("fecab"),
                    String::from("cgb"),
                    String::from("gbdefca"),
                    String::from("cg"),
                    String::from("fgcdab"),
                    String::from("egfdb"),
                    String::from("bfceg"),
                ],
                vec![
                    String::from("gbdfcae"),
                    String::from("bgc"),
                    String::from("cg"),
                    String::from("cgb"),
                ],
            ),
            (
                vec![
                    String::from("gcafb"),
                    String::from("gcf"),
                    String::from("dcaebfg"),
                    String::from("ecagb"),
                    String::from("gf"),
                    String::from("abcdeg"),
                    String::from("gaef"),
                    String::from("cafbge"),
                    String::from("fdbac"),
                    String::from("fegbdc"),
                ],
                vec![
                    String::from("fgae"),
                    String::from("cfgab"),
                    String::from("fg"),
                    String::from("bagce"),
                ],
            ),
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(26, part1(&input),);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(61229, part2(&input),);
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "}
    }
}
