use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
fn part1(lines: &[String]) -> usize {
    let mut score = 0;
    for line in lines {
        let mut sequence = vec![];
        for char in line.chars() {
            match char {
                '(' => sequence.push(')'),
                '[' => sequence.push(']'),
                '<' => sequence.push('>'),
                '{' => sequence.push('}'),
                _ => match sequence.pop() {
                    None => continue,
                    Some(expected) => {
                        if char.eq(&expected) {
                            continue;
                        }
                        match char {
                            ')' => score += 3,
                            ']' => score += 57,
                            '}' => score += 1197,
                            '>' => score += 25137,
                            _ => unreachable!(),
                        };
                        break;
                    }
                },
            }
        }
    }
    score
}

#[aoc(day10, part2)]
fn part2(lines: &[String]) -> usize {
    let mut scores = vec![];
    'outer: for line in lines {
        let mut sequence = vec![];
        for char in line.chars() {
            match char {
                '(' => sequence.push(')'),
                '[' => sequence.push(']'),
                '<' => sequence.push('>'),
                '{' => sequence.push('}'),
                _ => match sequence.pop() {
                    None => break,
                    Some(expected) => {
                        if char.eq(&expected) {
                            continue;
                        }
                        continue 'outer;
                    }
                },
            }
        }
        let mut score = 0;
        sequence.reverse();
        for char in sequence {
            score *= 5;
            match char {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => unreachable!(),
            }
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<String> = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(26397, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(288957, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        "}
    }
}
