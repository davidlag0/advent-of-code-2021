/*
--- Day 4: Giant Squid ---

You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

Maybe it wants to play bingo?

Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

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

After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

Finally, 24 is drawn:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).

The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.

To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
*/

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(board: Vec<Vec<i32>>) -> Board {
        Board { board }
    }

    pub fn sum(&self) -> i32 {
        let mut sum = 0;

        for row in &self.board {
            for item in row {
                if item > &-1 {
                    sum += item;
                }
            }
        }
        sum
    }

    pub fn is_winner(&self) -> bool {
        for row in &self.board {
            if row.iter().sum::<i32>() == -5 {
                return true;
            };
        }

        for column in 0..5 {
            let column_sum = self.board.iter().fold(0, |acc, x| acc + x[column]);
            if column_sum == -5 {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct BingoSubsystem {
    pub numbers: Vec<i32>,
    pub boards: Vec<Board>,
    pub last_number_drawn: i32,
}

impl BingoSubsystem {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|number| number.parse::<i32>().ok())
            .collect();

        // Skip empty line between the drawn numbers and the bingo boards.
        lines.next().unwrap();

        let raw_boards = lines
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|number| number.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let mut bingo_boards: Vec<Board> = Vec::new();

        for card in raw_boards.split(|vector| vector.is_empty()) {
            bingo_boards.push(Board::new(card.to_vec()));
        }

        Self {
            numbers,
            boards: bingo_boards,
            last_number_drawn: -1,
        }
    }

    pub fn find_winner_board(&self) -> Option<&Board> {
        self.boards.iter().find(|&board| board.is_winner())
    }

    pub fn draw(&mut self) -> Result<&Vec<i32>, &str> {
        if self.numbers.is_empty() {
            return Err("No more numbers to draw");
        }

        for board in &mut self.boards {
            for row in board.board.iter_mut() {
                let mut drawn_number_index = None;

                for (index, number) in row.iter().enumerate() {
                    if number == (self.numbers.first().unwrap()) {
                        drawn_number_index = Some(index);
                    }
                }

                if let Some(number) = drawn_number_index {
                    row[number] = -1;
                }
            }
        }

        self.last_number_drawn = *self.numbers.first().unwrap();
        self.numbers = self.numbers.drain(1..).collect();

        Ok(&self.numbers)
    }
}

pub fn part1(input: &str) -> i32 {
    let mut bingo_subsystem = BingoSubsystem::new(input);
    let mut board_sum = 0;

    while bingo_subsystem.draw().is_ok() {
        let winner_board = bingo_subsystem.find_winner_board();
        match winner_board {
            Some(board) => {
                board_sum = board.sum();
                break;
            }
            _ => continue,
        };
    }

    board_sum * bingo_subsystem.last_number_drawn
}

pub fn part2(input: &str) -> i32 {
    let mut bingo_subsystem = BingoSubsystem::new(input);

    while bingo_subsystem.draw().is_ok() {
        if bingo_subsystem.boards.len() == 1 {
            break;
        }

        bingo_subsystem.boards.retain(|board| !board.is_winner());
    }

    bingo_subsystem.boards[0].sum() * bingo_subsystem.last_number_drawn
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2, BingoSubsystem};

    static TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
";

    #[test]
    fn test_bingo_subsystem() {
        let bingo_subsystem = BingoSubsystem::new(TEST_INPUT);

        assert_eq!(
            bingo_subsystem.numbers,
            Vec::<i32>::from([
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ])
        );
        assert_eq!(
            bingo_subsystem.boards[0].board,
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19]
            ]
        );
        assert_eq!(
            bingo_subsystem.boards[1].board,
            vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6]
            ]
        );
        assert_eq!(
            bingo_subsystem.boards[2].board,
            vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7]
            ]
        );
    }

    #[test]
    fn test_bingo_subsystem_find_winner_board() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);

        for _ in 0..12 {
            bingo_subsystem.draw().ok();
        }

        assert_eq!(
            bingo_subsystem.find_winner_board().unwrap().board,
            vec![
                vec![-1, -1, -1, -1, -1],
                vec![10, 16, 15, -1, 19],
                vec![18, 8, -1, 26, 20],
                vec![22, -1, 13, 6, -1],
                vec![-1, -1, 12, 3, -1]
            ]
        );
    }

    #[test]
    fn test_bingo_subsystem_is_row_winner() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);
        bingo_subsystem.boards[0].board[0] = vec![-1, -1, -1, -1, -1];

        assert!(bingo_subsystem.boards[0].is_winner());
    }

    #[test]
    fn test_bingo_subsystem_is_colum_winner() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);
        bingo_subsystem.boards[0].board[0] = vec![14, 21, 17, 24, -1];
        bingo_subsystem.boards[0].board[1] = vec![10, 16, 15, 9, -1];
        bingo_subsystem.boards[0].board[2] = vec![18, 8, 23, 26, -1];
        bingo_subsystem.boards[0].board[3] = vec![22, 11, 13, 6, -1];
        bingo_subsystem.boards[0].board[4] = vec![2, 0, 12, 3, -1];

        assert!(bingo_subsystem.boards[0].is_winner());
    }

    #[test]
    fn test_bingo_subsystem_draw() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);
        bingo_subsystem.draw().ok();

        assert_eq!(bingo_subsystem.boards[0].board[2][4], -1);
        assert_eq!(bingo_subsystem.boards[1].board[2][2], -1);
        assert_eq!(bingo_subsystem.boards[2].board[4][4], -1);
    }

    #[test]
    fn test_bingo_subsystem_draw_11_times() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);

        for _ in 0..11 {
            bingo_subsystem.draw().ok();
        }

        assert!(bingo_subsystem.find_winner_board().is_none());
    }

    #[test]
    fn test_bingo_subsystem_draw_12_times() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);

        for _ in 0..12 {
            bingo_subsystem.draw().ok();
        }

        assert!(bingo_subsystem.find_winner_board().is_some());
    }

    #[test]
    fn test_bingo_board_sum() {
        let mut bingo_subsystem = BingoSubsystem::new(TEST_INPUT);

        for _ in 0..12 {
            bingo_subsystem.draw().ok();
        }

        assert_eq!(bingo_subsystem.find_winner_board().unwrap().sum(), 188);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1924);
    }
}
