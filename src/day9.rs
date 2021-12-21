use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(heightmap: &[Vec<u8>]) -> usize {
    let mut risk_levels: Vec<usize> = vec![];
    let xsize = heightmap.len();
    let ysize = heightmap[0].len();
    for x in 0..xsize {
        for y in 0..ysize {
            let curval = &heightmap[x][y];
            // Up.
            if y > 0 && heightmap[x][y - 1] <= *curval {
                continue;
            }
            // Down.
            if y < ysize - 1 && heightmap[x][y + 1] <= *curval {
                continue;
            }
            // Left.
            if x > 0 && heightmap[x - 1][y] <= *curval {
                continue;
            }
            // Right.
            if x < xsize - 1 && heightmap[x + 1][y] <= *curval {
                continue;
            }
            risk_levels.push(*curval as usize + 1);
        }
    }
    risk_levels.into_iter().sum()
}

#[aoc(day9, part2)]
fn part2(heightmap: &[Vec<u8>]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<Vec<u8>> = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(15, part1(&input),);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(168, part2(&input),);
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        "}
    }
}
