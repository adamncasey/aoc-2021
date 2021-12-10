use std::collections::HashSet;

#[derive(Debug)]
struct BingoGame {
    draws: Vec<u16>,
    boards: Vec<BingoBoard>,
}

type BingoBoard = Vec<u16>;
const BOARD_ROWS: usize = 5;
const BOARD_COLS: usize = 5;

fn check_board(board: &BingoBoard, called_nums: &HashSet<u16>) -> bool {
    for row in 0..BOARD_ROWS {
        let mut missing = false;
        for col in 0..BOARD_COLS {
            if !called_nums.contains(&board[row * BOARD_ROWS + col]) {
                missing = true;
                break;
            }
        }

        if !missing {
            return true;
        }
    }

    for col in 0..BOARD_COLS {
        let mut missing = false;
        for row in 0..BOARD_ROWS {
            if !called_nums.contains(&board[row * BOARD_ROWS + col]) {
                missing = true;
                break;
            }
        }

        if !missing {
            return true;
        }
    }

    return false;
}

fn calc_board(board: &BingoBoard, called_nums: &HashSet<u16>) -> usize {
    let mut total = 0;

    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            if !called_nums.contains(&board[row * BOARD_ROWS + col]) {
                total += board[row * BOARD_ROWS + col] as usize;
            }
        }
    }

    dbg!(total);

    total
}

fn day4(game: &BingoGame) -> usize {
    let mut called_nums = HashSet::new();
    for x in &game.draws {
        called_nums.insert(*x);

        for board in &game.boards {
            if check_board(&board, &called_nums) {
                return calc_board(&board, &called_nums) * (*x as usize);
            }
        }
    }
    panic!("No board completed");
}

fn day4_2(game: &BingoGame) -> usize {
    let mut called_nums = HashSet::new();
    let board_nums: Vec<usize> = (0..game.boards.len()).collect();
    let mut boards: HashSet<usize> = HashSet::from_iter(board_nums.iter().cloned());

    for x in &game.draws {
        called_nums.insert(*x);

        let old_boards = boards.clone();

        boards.retain(|board| !check_board(&game.boards[*board], &called_nums));

        if boards.len() == 0 {
            return calc_board(
                &game.boards[*old_boards.iter().next().unwrap()],
                &called_nums,
            ) * (*x as usize);
        }
    }

    panic!("No board completed");
}

fn read_board(num_grid: &str) -> BingoBoard {
    num_grid
        .split_whitespace()
        .map(|x| str::parse::<u16>(x).unwrap())
        .collect()
}

fn read_game(input: String) -> BingoGame {
    let input: Vec<&str> = input.split("\n\n").collect();

    let draws: Vec<u16> = input[0]
        .split(",")
        .map(|x| str::parse::<u16>(x).unwrap())
        .collect();

    let boards: Vec<BingoBoard> = input[1..].iter().map(|x| read_board(x)).collect();

    BingoGame { draws, boards }
}

#[test]
fn day4_example() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
     2  0 12  3  7";

    let game = read_game(input.to_string());

    assert_eq!(day4(&game), 4512);
}

#[test]
fn day4_actual() {
    let input = std::fs::read_to_string("./input/day4.txt").unwrap();

    let game = read_game(input.to_string());

    assert_eq!(day4(&game), 49686);
}

#[test]
fn day4_2_example() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
     2  0 12  3  7";

    let game = read_game(input.to_string());

    assert_eq!(day4_2(&game), 1924);
}

#[test]
fn day4_2_actual() {
    let input = std::fs::read_to_string("./input/day4.txt").unwrap();

    let game = read_game(input.to_string());

    assert_eq!(day4_2(&game), 26878);
}
