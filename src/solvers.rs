use crate::lib::*;


pub trait OneStepSolver {
    fn solve(&self, board: Board) -> Board;
}

struct LastCel;
struct LastRemainingCellLine;
pub struct LastRemainingCellBlock;
struct LastPossibleNumber;


impl OneStepSolver for LastCel {
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
            valid = board.try_set(index, num);
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
            valid = board.try_set(index, num);
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
            valid = board.try_set(index, num);
            if !valid {
                continue
            }
            return board;
        }
        return board
    }
}


impl OneStepSolver for LastRemainingCellLine {
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
                    board.try_set(*possible.get(0).unwrap(), val);
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
                    board.try_set(*possible.get(0).unwrap(), val);
                    return board
                }
            }
        }

        return board;
    }
}


impl OneStepSolver for LastRemainingCellBlock {
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
                    board.try_set(*valid_spots.get(0).unwrap(), val);
                    return board
                }
            }
        }
        return board
    }
}

impl OneStepSolver for LastPossibleNumber {
    fn solve(&self, mut board: Board) -> Board {
        let mut values_in_rows: Subset;
        let mut values_in_columns: Subset;
        let mut union: Vec<u8>;

        for i in board.blanks() {
            values_in_rows = board.row_from_index(i);
            values_in_columns = board.column_from_index(i);
            union = values_in_rows.union(&values_in_columns);
            if union.len() != 8 {
                continue
            }

            for ii in (1..=9) {
                if !union.contains(&ii) {
                   continue
                }
                board.try_set(i, ii);
                return board
            }
        }

        return board
    }
}
