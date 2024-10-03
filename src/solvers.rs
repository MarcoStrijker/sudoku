#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};
use std::vec;
use crate::lib::*;

use itertools::{Combinations, Itertools, rev};


#[derive(Debug)]
pub enum Orientation {
    Row,
    Column,
    Block,
}

// This is an easy reference to return no strategies
const NO_STRATEGIES: Vec<Strategy> = vec![];

pub trait SolveProbabilities {
    fn name() -> String;
    
    fn orientations() -> Vec<Orientation>;

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy>;

    /// Gets the strategy for the solver and applies the strategies to the board. Runs
    /// the solver for its own orientations and for index 0-8
    ///
    /// ### Arguments
    ///     board (mut Board): the sudoku
    ///
    /// ### Returns
    ///     Board: the adjusted sudoku
    fn get_and_apply_strategies(mut board: Board) -> Board {
        let mut strategies: Vec<Strategy> = Self::get_strategies(&board);

        for s in strategies {
            board.apply_strategy(s)
        }

        return board;
    }

    fn get_strategies(board: &Board) -> Vec<Strategy> {
        let mut strategies: Vec<Strategy> = vec![];
        for orientation in Self::orientations() {
            for i in 0..9 {
                strategies.extend(Self::logic(&board, &orientation, i));
            }
        }

        // for s in &strategies {
        //     s.print();
        // }

        return strategies;
    }

    fn apply_strategies(mut board: Board, strategies: Vec<Strategy>) -> Board {
        for s in strategies {
            board.apply_strategy(s)
        }

        return board;
    }

    /// Creates a default subset based on orientation and
    fn create_subset(board: &Board, orientation: &Orientation, index: u8) -> Subset {
        return match orientation {
            Orientation::Row => { board.row(index) },
            Orientation::Column => { board.column(index) },
            Orientation::Block => { board.block(index) },
        };
    }
}


/// LastRemainingCell
///
/// This deletes the probabilities of solved cells
pub struct LastRemainingCell;

impl SolveProbabilities for LastRemainingCell {
    fn name() -> String {
        return String::from("LastRemainingCell")
    }
    
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column, Orientation::Block];
    }

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy> {

        let mut strategies: Vec<Strategy> = vec![];
        let subset: Subset = Self::create_subset(board, orientation, index);
        let values_solved: HashSet<u8> = subset.values_solved();

        if values_solved.is_empty() {
            return strategies
        }

        for i in subset.indices {
            if board.cells[i as usize].solved() {
                continue
            }

            let probabilities_to_delete: HashSet<u8> = values_solved
                .intersection(&board.cells[i as usize].as_set())
                .cloned()
                .collect();

            strategies.push(
                Strategy::new(
                    Self::name(),
                    HashMap::from(
                        [(i, probabilities_to_delete)]
                    )
                )
            );
        }
        return strategies
    }
}


pub struct Naked;

impl SolveProbabilities for Naked {
    fn name() -> String {
        return String::from("LastRemainingCell")
    }
    
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column, Orientation::Block];
    }

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy> {
        let mut strategies: Vec<Strategy> = vec![];
        let mut naked: Vec<u8>;
        let mut unique_numbers: HashSet<u8>;
        let mut other_cells: Vec<u8>;

        let subset: Subset = Self::create_subset(board, orientation, index);

        for k in (2..=4).rev() {
            for combination in subset.missing().iter().combinations(k) {
                unique_numbers = HashSet::<u8>::new();
                for c in &combination {
                    unique_numbers.extend(c.as_set())
                }

                if unique_numbers.len() != k {
                    continue
                }

                // We want to get the index of the naked cells
                naked = combination
                    .iter()
                    .map(|c| c.index)
                    .collect();

                // The same goes for the rest
                other_cells = subset.indices
                    .iter()
                    .filter(|index| !naked.contains(&index))
                    .map(|index| *index)
                    .collect();


                let hashmap: HashMap<u8, HashSet<u8>> = other_cells
                    .iter()
                    .map(|i| (*i, unique_numbers.clone()))
                    .collect();

                strategies.push(
                    Strategy::new(
                        Self::name(),
                        hashmap
                    )
                )

                // board.remove_probabilities_from_cells(other_cells,
                //                                       unique_numbers.iter().map(|n| *n).collect_vec())
            }
        }

        return strategies
    }
}

pub struct Hidden;

impl SolveProbabilities for Hidden {
    fn name() -> String {
        return String::from("Hidden")
    }

    fn orientations() -> Vec<Orientation> {
        vec![Orientation::Row, Orientation::Column, Orientation::Block]
    }

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy> {
        let mut unique_numbers: HashSet<u8>;
        let mut indices_combinations: Vec<u8>;
        let mut other_cells: Vec<u8>;
        let mut strategies: Vec<Strategy> = vec![];

        // Iterate over all 9 rows, columns and blocks
        let subset: Subset = Self::create_subset(board, orientation, index);

        for k in (1..=4).rev() {
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

                let mut hashmap: HashMap<u8, HashSet<u8>> = HashMap::new();

                for c in combination {
                    let set: HashSet<u8> = c.as_set().difference(&possibly_hidden).map(|p| *p).collect();
                    hashmap.insert(c.index, set);
                }

                // // Now update the cells that are part of the identified combination
                // for cell in combination {
                //     board.cells[cell.index as usize].probabilities.retain(|&p| possibly_hidden.contains(&p));
                // }
                strategies.push(
                    Strategy::new(
                        Self::name(),
                        hashmap
                    )
                )

            }
        }
        return strategies
    }
}

/// Pointing uses block probabilities to eliminate probabilities for row/columns
///
/// When a block has no three and all three probabilities are on row 2, this
/// means in adjacent blocks no three is probable on row 2
pub struct Pointing;

impl SolveProbabilities for Pointing {
    fn name() -> String {
        String::from("LastRemainingCell")
    }
    
    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column];
    }

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy> {
        let subset: Subset;
        let missing: Vec<Cell>;
        let mut strategies: Vec<Strategy> = vec![];
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
            _ => panic!("Only Row/Column orientation is allowed with Pointing strategy")
        };

        subset = board.block(index);

        if subset.is_solved() {
            return strategies
        }

        missing = subset.missing();
        let values_solved: HashSet<u8> = subset.values_solved();

        for p in 1..=9 {
            if values_solved.contains(&p) {
                continue
            }

            missing_line = missing
                .clone()
                .into_iter()
                .filter(|c| c.contains(&p) && !c.solved())
                .map(|c| get_row_or_colum_index(&c))
                .collect::<HashSet<u8>>()
                .into_iter()
                .collect();

            // When there are probabilities in multiple rows within a block
            // the pointing strategy won't work
            if missing_line.len() != 1 {
                continue
            }

            let hashmap: HashMap<u8, HashSet<u8>> = get_line(board, missing_line[0])
                .cells
                .iter()
                .filter(|c| c.block() != index && !c.solved())
                .map(|c| (c.index, HashSet::from([p])))
                .collect();

            strategies.push(
                Strategy::new(
                    Self::name(),
                    hashmap
                )
            );
        }

        return strategies
    }
}


pub struct BoxLineReduction;

impl SolveProbabilities for BoxLineReduction {
    fn name() -> String {
        return String::from("BoxLineReduction")
    }

    fn orientations() -> Vec<Orientation> {
        return vec![Orientation::Row, Orientation::Column]
    }

    fn logic(board: &Board, orientation: &Orientation, index: u8) -> Vec<Strategy> {
        let mut strategies: Vec<Strategy> = vec![];
        let subset = Self::create_subset(board, orientation, index);

        if subset.is_solved() {
            return strategies
        }

        let get_row_or_colum_index: fn(&Cell) -> u8;

        // Create function that have gets rows/columns respective to the orientation
        match orientation {
            Orientation::Row => {
                get_row_or_colum_index = |c: &Cell | -> u8 {c.row()};
            }
            Orientation::Column => {
                get_row_or_colum_index = |c: &Cell | -> u8 {c.column()};
            },
            _ => panic!("Only Row/Column orientation is allowed with Pointing strategy")
        };

        for p in 1..=9 {
            let block_indices: Vec<u8> = subset.missing()
                .iter()
                .filter(|c| c.contains(&p) && !c.solved())
                .map(|c| c.block())
                .collect();

            if block_indices
                .iter()
                .map(|n| *n)
                .unique()
                .collect::<Vec<u8>>()
                .len() != 1 || block_indices.len() <= 1 {
                continue
            }

            let block: Subset = board.block(block_indices[0]);

            if block.is_solved() {
                continue
            }

            let _strategy: HashMap<u8, HashSet<u8>> = block
                .cells
                .iter()
                .filter(|c| get_row_or_colum_index(&c) == index && c.contains(&p))
                .map(|c| (c.index, HashSet::from([p])))
                .collect();

            let hashmap: HashMap<u8, HashSet<u8>> = block
                .cells
                .iter()
                .filter(|c| get_row_or_colum_index(c) != index && !c.solved() && c.contains(&p))
                .map(|c| (c.index, HashSet::from([p])))
                .collect();

            if hashmap.is_empty() {
                continue
            }

            strategies.push(
                Strategy::new(
                    Self::name(),
                    hashmap.clone()
                )
            );
        }
        return strategies
    }
}
