use std::fs;
use crate::lib::Board;


/// Solves a sudoku with brute-force
///
/// ### Args:
///     board (Board): The board that needs to be solved
///
/// ### Returns:
///     The solved board
#[allow(dead_code)]
pub fn brute_force(mut board: Board) -> Board {
    let blanks: Vec<u8> = board.blanks();
    if blanks.is_empty() {
        return board
    }
    let mut blank_index: usize = 0;
    let mut addition: u8 = 1;
    let mut history: Vec<Board> = Vec::with_capacity(blanks.len());

    let mut board_index = blanks[blank_index];
    let mut current_solution: u8 = board.get(board_index);

    while !board.solved() {
        if current_solution + addition > 9 {
            if blank_index == 0 {
                // No solution exists
                break;
            }
            blank_index -= 1;
            addition = 1;
            board_index = blanks[blank_index];
            current_solution = board.get(board_index);
            board = history[blank_index].clone();
            continue;
        }

        let valid: bool = board.try_set(board_index, current_solution + addition);

        if !valid {
            addition += 1;
            continue;
        }

        if blank_index < history.len() {
            history[blank_index] = board.clone();
        } else if blank_index + 1 == blanks.len() {
            break
        } else {
            history.push(board.clone());
        }

        blank_index += 1;
        addition = 1;
        board_index = blanks[blank_index];
        current_solution = board.get(board_index);
    }

    return board
}


/// Returns a vector with vectors, containing the puzzle
/// and solution of that puzzle
///
/// ### Returns:
///     A Vector containing a Vector that represents a sudoku,
///     the first element is the puzzle and the second the solution
///
#[allow(dead_code)]
pub fn import_puzzles_from_file() -> Vec<Vec<String>> {
    return fs::read_to_string(r"puzzles.txt")
        .expect("It should read the file")
        .lines()
        .map(|x| x
            .to_string()
            .split(", ")
            .map(|x| x.to_string())
            .collect())
        .collect();
}
