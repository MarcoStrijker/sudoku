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


trait SolveProbabilities {

    fn calculate(board: &mut Board) -> Board;
}

struct IntersectionRemoval;

impl SolveProbabilities for IntersectionRemoval {
    fn calculate(board: &mut Board) -> Board {
        for i in board.blanks() {
            for ii in board.row_from_index(i).values_solved() {
                &board.probabilities[i as usize].remove(ii as usize);
            }
            for ii in board.column_from_index(i).values_solved() {
                &board.probabilities[i as usize].remove(ii as usize);
            }
            for ii in board.block_from_index(i).values_solved() {
                &board.probabilities[i as usize].remove(ii as usize);
            }
        }
        return board.clone();
    }
}

struct Naked;

impl SolveProbabilities for Naked {
    fn calculate(board: &mut Board) -> Board {
        for i in 0..9 {
            let mut subset = board.row_from_index(i);
            let mut new_probabilities = Vec::new();
            for (ii, probabilities) in subset.probabilities.iter().enumerate() {
                if probabilities.len() != 2 {
                    new_probabilities.push(probabilities.clone());
                    continue;
                }

                let count = subset.probabilities
                    .iter()
                    .filter(|&x| x == probabilities)
                    .count();

                if count != 2 {
                    new_probabilities.push(probabilities.clone());
                    continue;
                }

                let new_probability = probabilities
                    .iter()
                    .filter(|x| !subset.probabilities.iter().any(|p| p != probabilities && p.contains(x)))
                    .cloned()
                    .collect();

                new_probabilities.push(new_probability);
            }
            subset.probabilities = new_probabilities;
        }

        board.clone()
    }
}
