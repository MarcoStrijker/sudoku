mod lib;
use lib::*;

trait SolveOneCell {
    fn solve(&self, board: Board) -> Board;
}

struct MissingCell;
struct InferenceMissingCellsLine;

impl SolveOneCell for MissingCell {
    fn solve(&self, mut board: Board) -> Board {
        let mut num: u8;
        let mut index: u8;
        let mut valid: bool;
        let mut subset: SubSet;
        let mut missing_values: Vec<u8>;

        for i in 0..8 {
            subset = board.row(i);
            missing_values = subset.values_missing();
            if missing_values.len() != 1 {
                continue
            }
            num = *missing_values.get(0).unwrap();
            index = *subset.indices_missing().get(0).unwrap();
            valid = board.set_in_row(i, index, num);
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
            valid = board.set_in_row(i, index, num);
            if !valid {
                continue
            }
            return board;
        }

        for i in 0..8 {
            subset = board.quadrant(i);
            missing_values = subset.values_missing();
            if missing_values.len() != 1 {
                continue
            }
            num = *missing_values.get(0).unwrap();
            index = *subset.indices_missing().get(0).unwrap();
            valid = board.set_in_row(i, index, num);
            if !valid {
                continue
            }
            return board;
        }
        return board
    }
}


impl SolveOneCell for InferenceMissingCellsLine {
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
    let start = String::from("672003004031000250040000013107040000390000045200075106005096378060508009900007501");
    // let end = String::from("695127304138459672724836915851264739273981546946573821317692458489715263562348197");
    let b = Board::from_string(&start);
    b.print_board();
    let mut new_b = InferenceMissingCellsLine.solve(b);
    while !new_b.solved() {
        new_b = InferenceMissingCellsLine.solve(new_b);
        new_b = MissingCell.solve(new_b);
    }

    new_b.print_board();
}
