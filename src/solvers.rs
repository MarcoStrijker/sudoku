use crate::lib::*;


pub trait DirectSolvers {
    fn solve(&self, board: Board) -> Board;
}

struct LastCel;
struct LastRemainingCellLine;
pub struct LastRemainingCellBlock;
struct LastPossibleNumber;


impl DirectSolvers for LastCel {
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
            num = missing_values[0];
            index = subset.indices_missing()[0];
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
            num = missing_values[0];
            index = subset.indices_missing()[0];
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
            num = missing_values[0];
            index = subset.indices_missing()[0];
            valid = board.try_set(index, num);
            if !valid {
                continue
            }
            return board;
        }
        return board
    }
}


impl DirectSolvers for LastRemainingCellLine {
    fn solve(&self, mut board: Board) -> Board {
        let mut missing_values: Vec<u8>;
        let mut missing_indices: Vec<u8>;
        let mut possible: Vec<u8>;

        // Look through rows
        for i in 0..9 {
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
                    board.try_set(possible[0], val);
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
                    board.try_set(possible[0], val);
                    return board
                }
            }
        }

        return board;
    }
}


impl DirectSolvers for LastRemainingCellBlock {
    fn solve(&self, mut board: Board) -> Board {
        let mut subset: Subset;
        let mut valid_spots: Vec<u8>;

        for i in 0..9 {
            // Check if anything needs to be solved
            subset = board.block(i);
            if !subset.has_missing() {
                continue
            }

            for val in subset.values_missing() {
                // Collect th valid spots
                // At the end, there will be checked if only
                // one spot is valid, this is the one we're interested in
                valid_spots = Vec::<u8>::new();
                for ii in subset.indices_missing() {
                    if board.row_from_index(ii).contains(&val)
                        || board.column_from_index(ii).contains(&val) {
                        continue
                    }

                    // If missing values not in the row and column
                    // of the cell, then it is a valid spot for the solution
                    valid_spots.push(ii);
                }

                // If only one spot is valid, we know the solution
                if valid_spots.len() == 1 {
                    board.try_set(valid_spots[0], val);
                    return board
                }
            }
        }
        return board
    }
}

impl DirectSolvers for LastPossibleNumber {
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

            for ii in 1..=9 {
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


pub trait SolveProbabilities {

    fn calculate(board: &mut Board) -> Board;
}


pub struct LastRemainingCell;

impl SolveProbabilities for LastRemainingCell {
    fn calculate(board: &mut Board) -> Board {
        let mut values_solved: Vec<u8>;

        for i in 0..9 {
            for subset in vec![board.row_from_index(i), board.column_from_index(i), board.block_from_index(i)] {
                values_solved = subset.values_solved();

                if values_solved.len() == 0 {
                    continue
                }

                for ii in subset.indices {
                    if board.probabilities[ii as usize].len() == 1 {
                        continue
                    }

                    // Delete solved numbers from cells
                    board.probabilities[ii as usize] = board.probabilities[ii as usize]
                        .clone()
                        .iter()
                        .filter(|x| !values_solved.contains(x))
                        .map(|x| *x)
                        .collect();
                }
            }
        }

        return board.clone();
    }
}

pub struct Naked;

impl SolveProbabilities for Naked {
    fn calculate(board: &mut Board) -> Board {
        let mut board_index: usize;
        let mut count: usize;
        let mut probability_index = vec![];
        let mut naked: Vec<u8>;

        for i in 0..9 {
            for subset in vec![board.row_from_index(i), board.column_from_index(i), board.block_from_index(i)] {
                probability_index = Vec::new();
                naked = Vec::new();
                for (ii, probabilities) in subset.probabilities.iter().enumerate() {
                    board_index = subset.indices[ii] as usize;

                    // Currently only naked pairs are supported
                    if probabilities.len() != 2 {
                        continue;
                    }

                    // Count how many cell share the same probabilities
                    count = subset.probabilities
                        .iter()
                        .filter(|&x| x == probabilities)
                        .count();

                    if count != 2 {
                        continue;
                    }

                    // Resolve probabilities
                    // Delete these specific probabilities
                    for p in probabilities {
                        if naked.contains(p) {
                            continue
                        }

                        naked.push(*p)
                    }

                    probability_index.push(board_index)
                }

                for ii in subset.indices {
                    // Don't change for the naked cells
                    if probability_index.contains(&(ii as usize)) {
                        continue
                    }

                    // Delete the solved probabilities
                    board.probabilities[ii as usize] = board.probabilities[ii as usize]
                        .iter()
                        .filter(|x| !naked.contains(x))
                        .map(|x| *x as u8)
                        .collect();
                }
            }
        }

        return board.clone()
    }
}
