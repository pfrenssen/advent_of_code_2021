use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Board {
    numbers: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
    winner: bool,
}

impl Board {
    fn mark_number(&mut self, number: u8) {
        'outer: for x in 0..5 {
            for y in 0..5 {
                if self.numbers[x][y] == number {
                    self.marks[x][y] = true;
                    break 'outer;
                }
            }
        }
        if self.has_horizontal_line() || self.has_vertical_line() {
            self.winner = true;
        }
    }

    const fn is_winner(&self) -> bool {
        self.winner
    }

    fn has_horizontal_line(&self) -> bool {
        for y in 0..5 {
            let mut c = 0;
            for x in 0..5 {
                if self.marks[x][y] {
                    c += 1;
                }
            }
            if c == 5 {
                return true;
            }
        }
        false
    }

    fn has_vertical_line(&self) -> bool {
        for x in 0..5 {
            let mut c = 0;
            for y in 0..5 {
                if self.marks[x][y] {
                    c += 1;
                }
            }
            if c == 5 {
                return true;
            }
        }
        false
    }

    fn get_unmarked_total(&self) -> usize {
        let mut total = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.marks[x][y] {
                    total += self.numbers[x][y] as usize;
                }
            }
        }
        total
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines: Vec<&str> = input.lines().collect();
    let values = lines
        .drain(0..1)
        .collect::<String>()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    // Skip the empty line between the values and boards.
    lines.drain(0..1);

    let mut boards = vec![];
    let mut board = [[0_u8; 5]; 5];
    let mut i = 0;

    for line in lines {
        if line.is_empty() {
            // An empty line indicates that a new board will start. Add the completed board to the
            // list, and initialize a new one.
            boards.push(Board {
                numbers: board,
                marks: [[false; 5]; 5],
                winner: false,
            });
            board = [[0_u8; 5]; 5];
            i = 0;
        } else {
            let row = line
                .split_whitespace()
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            board[i][..5].clone_from_slice(&row[..5]);
            i += 1;
        }
    }
    boards.push(Board {
        numbers: board,
        marks: [[false; 5]; 5],
        winner: false,
    });
    (values, boards)
}

#[aoc(day4, part1)]
fn part1(game: &(Vec<u8>, Vec<Board>)) -> usize {
    let (numbers, mut boards) = game.clone();
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.is_winner() {
                return number as usize * board.get_unmarked_total();
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
fn part2(game: &(Vec<u8>, Vec<Board>)) -> usize {
    let (numbers, mut boards) = game.clone();
    let mut remaining_winners = boards.len();
    for number in numbers {
        for board in boards.iter_mut() {
            if !board.is_winner() {
                board.mark_number(number);
                if board.is_winner() {
                    remaining_winners -= 1;
                    if remaining_winners == 0 {
                        return number as usize * board.get_unmarked_total();
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: (Vec<u8>, Vec<Board>) = (
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            vec![
                Board {
                    numbers: [
                        [22, 13, 17, 11, 0],
                        [8, 2, 23, 4, 24],
                        [21, 9, 14, 16, 7],
                        [6, 10, 3, 18, 5],
                        [1, 12, 20, 15, 19],
                    ],
                    marks: [[false; 5]; 5],
                    winner: false,
                },
                Board {
                    numbers: [
                        [3, 15, 0, 2, 22],
                        [9, 18, 13, 17, 5],
                        [19, 8, 7, 25, 23],
                        [20, 11, 10, 24, 4],
                        [14, 21, 16, 12, 6],
                    ],
                    marks: [[false; 5]; 5],
                    winner: false,
                },
                Board {
                    numbers: [
                        [14, 21, 17, 24, 4],
                        [10, 16, 15, 9, 19],
                        [18, 8, 23, 26, 20],
                        [22, 11, 13, 6, 5],
                        [2, 0, 12, 3, 7],
                    ],
                    marks: [[false; 5]; 5],
                    winner: false,
                },
            ],
        );
        assert_eq!(expected, parse_input(get_test_input()));
    }
    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(4512, part1(&input),);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(1924, part2(&input),);
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "}
    }
}
