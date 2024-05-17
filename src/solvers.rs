use std::collections::HashSet;
use std::vec;
use crate::lib::*;


pub trait DirectSolvers {
    fn solve(&self, board: Board) -> Board;
}

struct LastCel;
struct LastRemainingCellLine;
pub struct LastRemainingCellBlock;
struct LastPossibleNumber;


enum Orientation {
    Row,
    Column,
    Block
}



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

        for i in 0..9 {
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


pub struct LastRemainingCell;

impl LastRemainingCell {
    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut values_solved: Vec<u8>;
        let mut subset: Subset;


        for i in 0..9 {
            subset = match orientation {
                Orientation::Row => {board.row_from_index(i)},
                Orientation::Column => {board.column_from_index(i)},
                Orientation::Block => {board.block_from_index(i)}
            };

            values_solved = subset.values_solved();

            if values_solved.len() == 0 {
                continue
            }

            for ii in subset.indices {
                if board.cells[ii as usize].solved() {
                    continue
                }

                // Delete solved numbers from cells
                board.cells[ii as usize].probabilities = board.cells[ii as usize].probabilities
                    .clone()
                    .iter()
                    .filter(|x| !values_solved.contains(x))
                    .map(|x| *x)
                    .collect();
            }
        }

        return board.clone();
    }

    pub fn calculate(board: &mut Board) -> Board {
        for orientation in vec![Orientation::Row, Orientation::Column, Orientation::Block] {
            *board = Self::logic(board, orientation);
        }
        return board.clone()
    }
}

pub struct Naked;

impl Naked {
    pub fn calculate(board: &mut Board) -> Board {
        for orientation in vec![Orientation::Row, Orientation::Column] {
            *board = Self::logic(board, orientation);
        }
        return board.clone()
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut subset: Subset;
        let mut board_index: usize;
        let mut count: usize;
        let mut probability_index: Vec<usize>;
        let mut naked: Vec<u8>;

        for i in 0..9 {
            subset = match orientation {
                Orientation::Row => { board.row_from_index(i) },
                Orientation::Column => { board.column_from_index(i) },
                Orientation::Block => { board.block_from_index(i) }
            };
            probability_index = Vec::new();
            naked = Vec::new();
            for (ii, cell) in subset.cells.iter().enumerate() {
                board_index = subset.indices[ii] as usize;

                // Currently only naked pairs are supported
                if cell.probabilities.len() != 2 {
                    continue;
                }

                // Count how many cell share the same probabilities
                count = subset.cells
                    .iter()
                    .filter(|&x| x.probabilities == cell.probabilities)
                    .count();

                if count != 2 {
                    continue;
                }

                // Resolve probabilities
                // Delete these specific probabilities
                for p in &cell.probabilities {
                    if naked.contains(&p) {
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
                board.cells[ii as usize].probabilities = board.cells[ii as usize].probabilities
                    .iter()
                    .filter(|x| !naked.contains(x))
                    .map(|x| *x as u8)
                    .collect();
            }
        }
        return board.clone()
    }
}

pub struct Hidden;

impl Hidden {

    pub fn calculate(board: &mut Board) -> Board {
        for orientation in vec![Orientation::Row, Orientation::Column] {
            *board = Self::logic(board, orientation);
        }
        return board.clone()
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut subset: Subset;
        let mut missing: Vec<Cell>;
        let mut counts: Vec<u8>;
        let mut index: u8;

        // Iterate over all 9 rows, columns and blocks
        for i in 0..9 {
            subset = match orientation {
                Orientation::Row => { board.row_from_index(i) },
                Orientation::Column => { board.column_from_index(i) },
                Orientation::Block => { board.block_from_index(i) }
            };

            // Get the missing cells
            missing = subset.missing();

            // Get all single solutions
            counts = (1..=9)
                .map(|x| missing
                    .iter()
                    .filter(|c| c.probabilities.contains(&x))
                    .count() as u8
                )
                .collect();

            println!("{:?}", counts);
            for r in &subset.cells {
                print!("{:?}", r.probabilities);
            }
            println!();

            // Loop over the counts
            // If probabilities are found that are present once
            // This means this cell has to be that solution
            for (i, count) in counts.iter().enumerate() {
                if *count != 1 {
                    continue
                }

                // Get the index of the cell with the one probability
                index = subset.cells
                    .iter()
                    .filter(|c| c.contains(&((i + 1) as u8)))
                    .map(|c| c.index)
                    .collect::<Vec<u8>>()[0];

                // Set the solution
                board.cells[index as usize].set(&((i + 1) as u8))
            }
        }

        return board.clone()
    }
}


pub struct Pointing;

impl Pointing {

    pub fn calculate(board: &mut Board) -> Board {
        for orientation in vec![Orientation::Row, Orientation::Column] {
            *board = Self::logic(board, orientation);
        }
        return board.clone()
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut subset: Subset;
        let mut missing: Vec<Cell>;
        let mut missing_line: Vec<u8>;

        let get_row_or_colum_index: fn(&Cell) -> u8;
        let get_line: fn(&Board, u8) -> Subset;

        // Create function that have gets rows/columns respective to the orientation
        match orientation {
            Orientation::Row => {
                get_row_or_colum_index = |c: &Cell | -> u8 {c.row()};
                get_line = |b: &Board, i: u8 | -> Subset {b.row(i)};
            }
            Orientation::Column => {
                get_row_or_colum_index = |c: &Cell | -> u8 {c.row()};
                get_line = |b: &Board, i: u8 | -> Subset {b.row(i)};
            },
            _ => panic!("Block operation not allowed with Pointing strategy")
        };

        for i in 0..9 {
            subset = board.block(i);
            missing = subset.missing();
            for p in 1..=9 {
                missing_line = missing
                    .clone()
                    .into_iter()
                    .filter(|c| c.contains(&p))
                    .map(|c| get_row_or_colum_index(&c))
                    .collect::<HashSet<u8>>()
                    .into_iter()
                    .collect();

                // When there are probabilities in multiple rows
                // the pointing strategy won't work
                if missing_line.len() != 1 {
                    continue
                }

                // Remove probabilities that are in the same row
                // Prevent removing in the focal block (subset)
                for c in get_line(board, missing_line[0]).cells.iter() {
                    if subset.indices.contains(&c.index) || c.solved() {
                        continue
                    }
                    board.cells[c.index as usize].remove(p);
                }
            }
        }

        return board.clone()
    }

}