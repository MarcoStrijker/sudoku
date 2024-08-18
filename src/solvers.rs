use std::collections::HashSet;
use std::vec;
use crate::lib::*;

use itertools::{Combinations, Itertools, rev};


#[derive(Debug)]
pub enum Orientation {
    Row,
    Column,
    Block
}


pub trait SolveProbabilities {
    fn orientations() -> Vec<Orientation>;

    fn logic(board: &mut Board, orientation: Orientation) -> Board;

    // Calculate for multiple orientations
    fn calculate(board: &mut Board) -> Board {
        for orientation in Self::orientations() {
            *board = Self::logic(board, orientation);
        }
        return board.clone();
    }
}


/// LastRemainingCell
///
/// This deletes the probabilities of solved cells
pub struct LastRemainingCell;

impl SolveProbabilities for LastRemainingCell {
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column, Orientation::Block];
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut values_solved: Vec<u8>;
        let mut subset: Subset;


        for i in 0..9 {
            subset = match orientation {
                Orientation::Row => {board.row(i)},
                Orientation::Column => {board.column(i)},
                Orientation::Block => {board.block(i)}
            };

            values_solved = subset.values_solved();

            if values_solved.is_empty() {
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
}

pub struct Naked;

impl SolveProbabilities for Naked {
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column, Orientation::Block];
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut subset: Subset;
        let mut naked: Vec<u8>;
        let mut unique_numbers: HashSet<u8>;
        let mut other_cells: Vec<u8>;

        for i in 0..9 {
            subset = match orientation {
                Orientation::Row => { board.row(i) },
                Orientation::Column => { board.column(i) },
                Orientation::Block => { board.block(i) }
            };

            for k in (2..=4).rev() {
                for combination in subset.missing().iter().combinations(k) {
                    unique_numbers = HashSet::<u8>::new();
                    for c in &combination {
                        unique_numbers.extend(c.as_set())
                    }

                    if unique_numbers.len() != k {
                        continue
                    }

                    naked = combination
                        .iter()
                        .map(|c| c.index)
                        .collect();

                    other_cells = subset.indices
                        .iter()
                        .filter(|index| !naked.contains(&index))
                        .map(|index| *index)
                        .collect();

                    board.remove_probabilities_from_cells(other_cells,
                                                          unique_numbers.iter().map(|n| *n).collect_vec())
                    }
                }
            }

        return board.clone()
    }
}

pub struct Hidden;

impl SolveProbabilities for Hidden {
    fn orientations() -> Vec<Orientation> {
        vec![Orientation::Row, Orientation::Column, Orientation::Block]
    }

    fn logic(board: &mut Board, orientation: Orientation) -> Board {
        let mut subset: Subset;
        let mut unique_numbers: HashSet<u8>;
        let mut indices_combinations: Vec<u8>;
        let mut other_cells: Vec<u8>;

        // Iterate over all 9 rows, columns and blocks
        for i in 0..9 {
            println!("{:?}", orientation);
            subset = match orientation {
                Orientation::Row => board.row(i),
                Orientation::Column => board.column(i),
                Orientation::Block => board.block(i),
            };

            for k in (2..=4).rev() {
                for combination in subset.missing().iter().combinations(k) {
                    unique_numbers = HashSet::<u8>::new();
                    for cell in &combination {
                        unique_numbers.extend(cell.as_set());
                    }
                    indices_combinations = combination.iter().map(|cell| cell.index).collect();

                    other_cells = subset.indices.iter()
                        .filter(|index| !indices_combinations.contains(index))
                        .cloned()
                        .collect();
                    let mut possibly_hidden = unique_numbers.clone();

                    for number in &unique_numbers {
                        if other_cells.iter().any(|&other_cell_index| board.cells[other_cell_index as usize].probabilities.contains(number)) {
                            possibly_hidden.remove(number);
                        }
                    }

                    // possibly_hidden
                    if possibly_hidden.len() != k {
                        continue;
                    }

                    // The number of matches from the possible hidden should be at least two
                    if combination.iter().any(|c| {
                        c.probabilities.iter().filter(|p| possibly_hidden.contains(p)).take(2).count() < 2
                    }) {
                        continue
                    }

                    println!("{:?} {:?}", possibly_hidden, k);
                    println!("indices: {:?}", indices_combinations);

                    // At this point we think the possibly hidden are hidden
                    for cell in &subset.missing() {
                        if indices_combinations.contains(&cell.index) {
                            continue;
                        }

                        if cell.as_set().intersection(&possibly_hidden).count() > 0 {
                            println!("A:{:?}", cell.index);
                            board.cells[cell.index as usize].probabilities.retain(|&p| possibly_hidden.contains(&p));
                        }
                    }

                    // Now update the cells that are part of the identified combination
                    for cell in combination {
                        board.cells[cell.index as usize].probabilities.retain(|&p| possibly_hidden.contains(&p));
                    }

                }
            }
        }
        board.clone()
    }
}

/// Pointing uses block probabilities to eliminate probabilities for row/columns
///
/// When a block has no three and all three probabilities are on row 2, this
/// means in adjacent blocks no three is probable on row 2
pub struct Pointing;

impl SolveProbabilities for Pointing {
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column];
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
                get_row_or_colum_index = |c: &Cell | -> u8 {c.column()};
                get_line = |b: &Board, i: u8 | -> Subset {b.column(i)};
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

                // When there are probabilities in multiple rows within a block
                // the pointing strategy won't work
                if missing_line.len() != 1 {
                    continue
                }

                // Remove probabilities that are in the same row
                for c in get_line(board, missing_line[0]).cells.iter() {
                    // Prevent removing in the focal block (subset)
                    if c.block() == i || c.solved() {
                        continue
                    }
                    board.cells[c.index as usize].remove(p);
                }
            }
        }

        return board.clone()
    }
}
