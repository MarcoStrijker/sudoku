mod lib;

use std::cmp::PartialEq;
use lib::*;

trait Solver {
    fn solve(&self, board: Board) -> Board;
}

struct LastCel;
struct LastRemainingCellLine;
struct LastRemainingCellBlock;

impl Solver for LastCel {
    fn solve(&self, mut board: Board) -> Board {
        let mut num: u8;
        let mut index: u8;
        let mut valid: bool;
        let mut subset: Subset;
        let mut missing_values: Vec<u8>;

        for i in 0..8 {
            subset = board.row(i);
            missing_values = subset.values_missing();
            if missing_values.len() != 1 {
                continue
            }
            num = *missing_values.get(0).unwrap();
            index = *subset.indices_missing().get(0).unwrap();
            valid = board.set(index, num);
            if !valid {
                continue
            }
            return board;
        }

        for i in 0..8 {
            subset = board.column(i);
            missing_values = subset.values_missing();
            if missing_values.len() != 1 {
                continue
            }
            num = *missing_values.get(0).unwrap();
            index = *subset.indices_missing().get(0).unwrap();
            valid = board.set(index, num);
            if !valid {
                continue
            }
            return board;
        }

        for i in 0..8 {
            subset = board.block(i);
            missing_values = subset.values_missing();
            if missing_values.len() != 1 {
                continue
            }
            num = *missing_values.get(0).unwrap();
            index = *subset.indices_missing().get(0).unwrap();
            valid = board.set(index, num);
            if !valid {
                continue
            }
            return board;
        }
        return board
    }
}


impl Solver for LastRemainingCellLine {
    fn solve(&self, mut board: Board) -> Board {
        let mut missing_values: Vec<u8>;
        let mut missing_indices: Vec<u8>;
        let mut possible: Vec<u8>;

        // Look through rows
        for i in (0..9) {
            missing_values = board.row(i).values_missing();
            if missing_values.is_empty() {
                continue
            }
            missing_indices = board.row(i).indices_missing();

            for val in missing_values {
                possible = Vec::<u8>::new();

                for ii in &missing_indices {
                    if !board.column_from_index(*ii).contains(&val) {
                        possible.push(*ii)
                    }
                }

                if possible.len() == 1 {
                    board.set(*possible.get(0).unwrap(), val);
                    return board
                }
            }
        }

        for i in (0..9) {
            missing_values = board.column(i).values_missing();
            if missing_values.is_empty() {
                continue
            }
            missing_indices = board.column(i).indices_missing();

            for val in missing_values {
                possible = Vec::<u8>::new();

                for ii in &missing_indices {
                    if !board.row_from_index(*ii).contains(&val) {
                        possible.push(*ii)
                    }
                }

                if possible.len() == 1 {
                    board.set(*possible.get(0).unwrap(), val);
                    return board
                }
            }
        }

        return board;
    }
}


impl Solver for LastRemainingCellBlock {
    fn solve(&self, mut board: Board) -> Board {
        let mut missing_indices;
        let mut missing_values;
        let mut subset: Subset;

        let mut column: Subset;
        let mut row: Subset;
        let mut valid_spots: Vec<u8>;

        for i in 0..9 {
            // Check if anything needs to be solved
            subset = board.block(i);
            if !subset.has_missing() {
                continue
            }

            // Get indices and values of the cells
            missing_indices = subset.indices_missing();
            missing_values = subset.values_missing();

            for val in missing_values {
                // Collect th valid spots
                // At the end, there will be checked if only
                // one spot is valid, this is the one we're interested in
                valid_spots = Vec::<u8>::new();
                for ii in &missing_indices {
                    row = board.row_from_index(*ii);
                    if row.contains(&val) {
                        continue
                    }
                    column = board.column_from_index(*ii);
                    if column.contains(&val) {
                        continue
                    }

                    // If missing values not in the row and column
                    // of the cell, then it is a valid spot for the solution
                    valid_spots.push(*ii);
                }

                // If only one spot is valid, we know the solution
                if valid_spots.len() == 1 {
                    board.set(*valid_spots.get(0).unwrap(), val);
                    return board
                }
            }
        }
        return board
    }
}

fn brute_force(mut board: Board) -> Board {
    let mut valid: bool;
    let mut current_index_board: u8;
    let mut current_solution: u8;
    let mut current_index: u8 = 0;
    let mut addition: u8 = 1;
    let mut count: u32 = 0;
    let mut solve_history: Vec<u32> = Vec::<u32>::new();

    // Get a vector with the index of the blank cells
    let blanks: Vec<u8> = board.blanks();

    current_index = 0;

    while !board.solved() {
        // Convert the index within the blanks to the board index
        // And fetch the filled in number
        current_index_board = blanks[current_index as usize];
        current_solution = board.get(current_index_board);

        // When all possible solutions are exhausted
        // Move to previous solution
        if current_solution + addition > 9 {
            board.rollback(solve_history[(current_index as usize) - 1]);
            current_index -= 1;
            addition = 1;
            continue
        }

        valid = board.set(current_index_board, current_solution + addition);

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


fn main() {
    let start = String::from("065370002000001370000640800097004028080090001100020940040006700070018050230900060");
    // let end = String::from("695127304138459672724836915851264739273981546946573821317692458489715263562348197");
    let b = Board::from_string(&start);
    b.print_board();
    let mut old_b: Board = b.clone();
    let mut new_b: Board;
    for i in 0..100 {
        old_b = InferenceBlock.solve(old_b);
    }
    // old_b.print_board()
    println!("{:?}", old_b.to_string())

}
