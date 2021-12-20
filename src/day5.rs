use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    s: Coord,
    e: Coord,
}

impl Line {
    const fn is_orthogonal(&self) -> bool {
        self.s.x == self.e.x || self.s.y == self.e.y
    }

    const fn get_x_range(&self) -> RangeInclusive<usize> {
        if self.s.x < self.e.x {
            self.s.x..=self.e.x
        } else {
            self.e.x..=self.s.x
        }
    }
    const fn get_y_range(&self) -> RangeInclusive<usize> {
        if self.s.y < self.e.y {
            self.s.y..=self.e.y
        } else {
            self.e.y..=self.s.y
        }
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Line {
                s: Coord {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                e: Coord {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(lines: &[Line]) -> usize {
    count_danger_zones(lines, false)
}

#[aoc(day5, part2)]
fn part2(lines: &[Line]) -> usize {
    count_danger_zones(lines, true)
}

fn count_danger_zones(lines: &[Line], check_diagonal: bool) -> usize {
    let mut grid = [[0_u8; 1000]; 1000];

    for line in lines {
        if line.is_orthogonal() {
            for x in line.get_x_range() {
                for y in line.get_y_range() {
                    // Ensure that we don't overflow.
                    if grid[x][y] < 255 {
                        grid[x][y] += 1;
                    }
                }
            }
        } else if check_diagonal {
            let mut x = line.s.x;
            let mut y = line.s.y;
            loop {
                // Ensure that we don't overflow.
                if grid[x][y] < 255 {
                    grid[x][y] += 1;
                }
                if x == line.e.x {
                    break;
                }
                if line.s.x < line.e.x {
                    x += 1;
                } else {
                    x -= 1;
                }
                if line.s.y < line.e.y {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    let mut result = 0;
    for row in &grid {
        for value in row {
            if *value > 1 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<Line> = vec![
            Line {
                s: Coord { x: 0, y: 9 },
                e: Coord { x: 5, y: 9 },
            },
            Line {
                s: Coord { x: 8, y: 0 },
                e: Coord { x: 0, y: 8 },
            },
            Line {
                s: Coord { x: 9, y: 4 },
                e: Coord { x: 3, y: 4 },
            },
            Line {
                s: Coord { x: 2, y: 2 },
                e: Coord { x: 2, y: 1 },
            },
            Line {
                s: Coord { x: 7, y: 0 },
                e: Coord { x: 7, y: 4 },
            },
            Line {
                s: Coord { x: 6, y: 4 },
                e: Coord { x: 2, y: 0 },
            },
            Line {
                s: Coord { x: 0, y: 9 },
                e: Coord { x: 2, y: 9 },
            },
            Line {
                s: Coord { x: 3, y: 4 },
                e: Coord { x: 1, y: 4 },
            },
            Line {
                s: Coord { x: 0, y: 0 },
                e: Coord { x: 8, y: 8 },
            },
            Line {
                s: Coord { x: 5, y: 5 },
                e: Coord { x: 8, y: 2 },
            },
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(12, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        "}
    }
}
