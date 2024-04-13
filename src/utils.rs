use std::fs;
use crate::lib::Board;


pub fn brute_force(mut board: Board) -> Board {
    /// Solves a sudoku with brute-force
    ///
    /// Args:
    ///     board (Board): The board that needs to be solved
    ///
    /// Returns:
    ///     The solved board
    ///
    let mut valid: bool;
    let mut current_index_board: u8;
    let mut current_solution: u8;
    let mut current_index: u8 = 0;
    let mut addition: u8 = 1;
    let mut count: u32 = 0;
    let mut solve_history: Vec<u32> = Vec::<u32>::new();

    // Get a vector with the index of the blank cells
    let blanks: Vec<u8> = board.blanks();

    while !board.solved() {
        // Convert the index within the blanks to the board index
        // And fetch the filled in number
        current_index_board = blanks[current_index as usize];
        current_solution = board.get(current_index_board);

        // When all possible solutions are exhausted
        // Move to previous solution, rollback the board
        if current_solution + addition > 9 {
            board.rollback(solve_history[(current_index as usize) - 1]);
            current_index -= 1;
            addition = 1;
            continue
        }

        valid = board.try_set(current_index_board, current_solution + addition);

        if !valid {
            addition += 1;
            continue
        }

        // Add new solution
        count += 1;
        solve_history.insert(current_index as usize, count);
        current_index += 1;
        addition = 1;
    }

    return board
}

pub fn import_puzzles_from_file() -> Vec<Vec<String>> {
    /// Returns a vector with vectors, containing the puzzle
    /// and solution of that puzzle
    ///
    /// Returns:
    ///     A Vector containing a Vector that represents a sudoku,
    ///     the first element is the puzzle and the second the solution
    ///
    return fs::read_to_string(r"puzzles.txt")
        .expect("It should read the file")
        .lines()
        .map(|x| x
            .to_string()
            .split(", ")
            .map(|x| x
                .to_string())
            .collect())
        .collect();
}
