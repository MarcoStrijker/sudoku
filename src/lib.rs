#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};
use itertools::Itertools;


/// Contains the formulas to get the index of the row, column and block
///
/// The input is the index of a certain cell
///
enum SubsetIndexToSubset {}

impl SubsetIndexToSubset {
    fn row(i: u8, iter: u8) -> u8 { i * 9 + iter }
    fn column(i: u8, iter: u8) -> u8 {
        iter * 9 + i
    }
    fn block(i: u8, iter: u8) -> u8 { 27 * (i / 3) + 3 * (i % 3) + (iter / 3) * 9 + (iter % 3) }
}


enum BoardIndexFormulas{}

#[allow(dead_code)]
impl BoardIndexFormulas {
    fn row(i: u8, iter: u8) -> u8 {
        (i / 9) * 9 + iter
    }
    fn column(i: u8, iter: u8) -> u8 {
        i % 9 + 9 * iter
    }
    fn block(i: u8, iter: u8) -> u8 { SubsetIndexToSubset::block(i / 3 - i / 9 * 3 + i / 27 * 3, iter) }
}

#[allow(dead_code)]
pub struct Subset {
    pub indices: [u8; 9],
    pub cells: [Cell; 9]
}


/// Initializes a subset of Cells from a board instance based on the requested
/// index and passed function. Eligible functions can be found in SubsetIndexToSubset
/// and BoardIndexFormulas
///
/// Args:
///     board (&Board): the board
///     i (u8): index, can be the index of the requested subset, or the
///             index of the cell on the board. This changeable through the
///             passed function
///     func (&dyn Fn(u8, u8) -> u8): formula that determines the cells
///
/// Returns:
///     Subset
#[allow(dead_code)]
impl Subset {
    fn from_board(board: &Board, i: u8, func: &dyn Fn(u8, u8) -> u8) -> Subset {
        return Subset {
            indices: (0..9)
                .map(|x| func(i, x))
                .collect::<Vec<u8>>()
                .try_into()
                .expect(""),
            cells: (0..9)
                .map(|x| board.cells[usize::from(func(i, x))].clone())
                .collect::<Vec<Cell>>()
                .try_into()
                .expect("")
        }
    }

    pub fn is_solved(&self) -> bool {
        return self.cells
            .iter()
            .all(|c| c.solved())
    }

    /// Wrapper for Vector.contains. Looks into the values of the
    /// subset if it contains the passed value
    ///
    /// ### Args:
    ///     value (u8): The value that should be checked
    ///
    /// ### Return:
    ///     True if the value is in self.values
    ///
    pub fn contains(&self, value: &u8) -> bool {
        return self.cells
            .iter()
            .filter(|c| c.solved())
            .any(|c| c.value() == *value)
    }

    pub fn missing(&self) -> Vec<Cell> {
        return self.cells
            .clone()
            .into_iter()
            .filter(|c| !c.solved())
            .collect::<Vec<Cell>>();
    }

    pub fn indices_missing(&self) -> Vec<u8> {
        return self.cells
            .iter()
            .filter(|c| !c.solved())
            .map(|c| c.index)
            .collect();
    }

    /// Get the values that are solved in Self
    ///
    /// ### Returns:
    ///     The solved values (Vec<u8>)
    pub fn values_solved(&self) -> HashSet<u8> {
        return self.cells
            .iter()
            .filter(|c| c.solved())
            .map(|c| c.value())
            .collect()
    }

    /// Returns all unique solved values that are in self or other
    ///
    /// # Args:
    ///     other (Subset): The other subset
    ///
    /// # Returns:
    /// A vector containing the solved values
    pub fn union(&self, other: &Self) -> Vec<u8> {
        let mut union: Vec<u8> = self.cells
            .iter()
            .filter(|c| c.solved())
            .map(|c| c.value())
            .collect::<Vec<u8>>();
        for v in other.cells
            .iter()
            .filter(|c| c.solved())
            .map(|c| c.value())
            .collect::<Vec<u8>>() {
            if v != 0 || union.contains(&v) {
                continue
            }
            union.push(v)
        }

        return union;
    }
}


pub struct Board {
    pub cells: [Cell; 81],
}

#[allow(dead_code)]
impl Board {

    /// Initialize board from a string
    pub fn from_string(string: &str) -> Board {
        let current_state: [Cell; 81] = string
            .chars()
            .enumerate()
            .map(|(i, char)| Cell::from_number(i, char.to_digit(10).unwrap() as u8))
            .collect::<Vec<Cell>>()
            .try_into()
            .expect("The string must be 81 characters");

        return Board {
            cells: current_state,
        };
    }

    /// Clones the board by initializing new board with cloned values
    ///
    /// ### Returns
    ///     Cloned board (Board)
    pub fn clone(&self) -> Board {
        return Board {
            cells: self.cells.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        return self.cells
            .iter()
            .map(|c| c.value().to_string())
            .collect();
    }

    /// ### Prints the board
    ///
    /// Prints the board in a human readable format, creates nice blocks.
    /// Also prints the percentage of the board that is solved, and the number of probabilities
    /// left in the board. This indicates the progression of a certain strategy.
    pub fn print_board(&self) {
        let string: String = self.to_string();
        let percentage_completed: f32 = self.cells
            .iter()
            .filter(|c| c.solved())
            .count() as f32 / 81f32 * 100f32;

        println!();
        println!("{:?} - {:?}%", self.uncertainty(), percentage_completed);
        for (i, s) in string.chars().enumerate() {
            if i != 0 && i % 9 == 0 {
                // After each row, print a new line
                println!();
            }

            if i != 0 && i % 27 == 0 {
                // After three rows, print a row separation
                println!(" —  —  —   —  —  —   —  —  — ");
            } else if i != 0 && i % 3 == 0 && i % 9 != 0 {
                // Print column separators
                print!("|")
            }

            // Print the solution
            print!(" {} ", s);
        }
        println!()
    }

    /// Returns the sum of the number of probabilities
    /// Acts as measure towards solving a puzzle
    ///
    /// ### Returns:
    ///     Number of probabilities (usize)
    fn uncertainty(&self) -> usize {
        return self.cells
            .iter()
            .map(|c| c.probabilities.len())
            .sum::<usize>()
    }

    /// ### Get a vector with the index of the blank cells
    ///
    /// ### Returns:
    ///    Vector with the index of the blank cells (Vec<u8>)
    pub fn blanks(&self) -> Vec<u8> {
        return self.cells
            .iter()
            .filter(|c| !c.solved())
            .map(|c| c.index)
            .collect();
    }

    /// Get the value of a cell
    ///
    /// ### Args:
    ///     index (u8): the index of the cell
    ///
    /// ### Returns:
    ///     The value of the cell (u8)
    pub fn get(&self, index: u8) -> u8 {
        if index > 80 {
            panic!("You've tried getting a number with a to high index. Not allowed")
        }
        return self.cells[index as usize].value()
    }

    fn set(&mut self, index: u8, solution: u8) {
        // Sets a solution into a cell. Changes the numbers and add the change to the history
        //
        // Args:
        //     index (u8): the index on the board in which you want the solution to be placed
        //     solution (u8): The solution 1-9
        //
        self.cells[index as usize].set(&solution);
    }

    /// Try to set a solution into a cell
    ///
    /// Args:
    ///     index (u8): the index on the board in which you want the solution to be placed
    ///     solution (u8): The solution 1-9
    ///
    /// Returns:
    ///     true if the set is valid, false if not (bool)
    pub fn try_set(&mut self, index: u8, solution: u8) -> bool {
        if index > 80 {
            panic!("You've tried setting a number with a to high index. Not allowed")
        }

        if !self.validate(index, solution) {
            return false;
        }

        // Set the solution
        self.set(index, solution);
        return true
    }

    /// Removes multiple probabilities from multiple cells
    ///
    /// ### Arguments
    ///     indices (Vec<u8>)
    ///     probabilities (Vec<u8>)
    pub fn remove_probabilities_from_cells(&mut self, indices: Vec<u8>, probabilities: Vec<u8>) {
        for i in indices {
            self.cells[i as usize].probabilities.retain(|p| !probabilities.contains(p))
        }
    }

    pub fn row(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &SubsetIndexToSubset::row)
    }

    pub fn column(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &SubsetIndexToSubset::column)
    }

    pub fn block(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &SubsetIndexToSubset::block)
    }

    pub fn row_from_index(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &BoardIndexFormulas::row)
    }

    pub fn column_from_index(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &BoardIndexFormulas::column)
    }

    pub fn block_from_index(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &BoardIndexFormulas::block)
    }

    fn validate(&self, index: u8, solution: u8) -> bool {
        return !(
            self.block_from_index(index).contains(&solution)
            || self.row_from_index(index).contains(&solution)
            || self.column_from_index(index).contains(&solution)
        )
    }

    pub fn apply_strategy(&mut self, strategy: Strategy) {
        for (i, probabilities) in strategy.remove {
            self.cells[usize::from(i)].probabilities.retain(|p| !probabilities.contains(p))
        }
    }

    /// Checks if the board is solved
    ///
    /// ### Returns:
    ///     true is the puzzle is solved, false if not
    ///
    pub fn solved(&self) -> bool {
        return self.cells
            .iter()
            .all(|c| c.solved())
    }

}

/// Equality implementation for Board
///
/// ### Arguments
///    other (&Self): other board to compare
///
/// ### Returns:
///    bool: if the board is the same
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        return self.cells == other.cells
    }
}


/// Cell
///
/// Contains the index of the cell and the probabilities. The probabilities are
/// a vector of u8. If the cell is solved, the vector will contain one value.
///
/// The cell also contains methods to get the row, column and block of the cell.
///
#[derive(Clone)]
#[derive(Debug)]
pub struct Cell {
    pub index: u8,
    pub probabilities: Vec<u8>
}

#[allow(dead_code)]
impl Cell {

    /// Creates a new cell
    ///
    /// The cell will have the index and the number as a probability. If the number is 0,
    /// this means that the cell is empty and the probabilities will be 1-9.
    ///
    /// ### Arguments:
    ///    i (usize): index of the cell
    ///    number (u8): number in the cell
    ///
    /// ### Returns
    ///   Cell
    fn from_number(i: usize, number: u8) -> Cell {
        return Cell {
            index: i as u8,
            probabilities: if number == 0 {vec![1,2,3,4,5,6,7,8,9]} else {vec![number]}
        }
    }

    /// Get the probabilities as a set
    ///
    /// This makers it easier to combine and compare sets
    ///
    /// ### Returns
    ///   HashSet<u8>: set of probabilities
    pub fn as_set(&self) -> HashSet<u8> {
        return HashSet::from_iter(self.probabilities.clone())
    }

    /// Get the index of the row
    ///
    /// For example, cell index 10 corresponds to row index 1, 18 to 2
    ///
    /// ### Returns
    ///    u8: index of the row
    pub fn row(&self) -> u8 {
        return self.index / 9
    }

    /// Get the index of the column
    ///
    /// For example, cell index 10 corresponds to column index 1
    ///
    /// ### Returns
    ///   u8: index of the column
    pub fn column(&self) -> u8 {
        return self.index % 9
    }

    /// Get the index of the block
    ///
    /// For example, cell index 8 corresponds to block index 2, but 10 to 1
    ///
    /// ### Returns
    ///   u8: index of the block
    pub fn block(&self) -> u8 {
        return self.index / 3 - self.index / 9 * 3 + self.index / 27 * 3
    }

    /// Check if the cell probabilities contain a value
    ///
    /// ### Arguments
    ///  value (&u8): the value that should be checked
    ///
    /// ### Returns
    ///   bool: if the value is in the probabilities
    pub fn contains(&self, value: &u8) -> bool {
        return self.probabilities.contains(value)
    }

    /// Force set a solution (probabilities will be a vector of one).
    /// No checks will be executed
    ///
    /// ### Arguments
    ///   value (&u8): the value that should be set
    pub fn set(&mut self, value: &u8) {
        self.probabilities = vec![*value]
    }

    /// Removes a probability from the probabilities
    ///
    /// ### Arguments
    ///   value (u8): the value that should be removed
    pub fn remove(&mut self, value: u8) {
        if !self.probabilities.contains(&value) {
            return;
        }

        self.probabilities.retain(|p| p != &value)
    }

    /// Get the value of the cell
    ///
    /// If the cell is solved, the value will be the solution, otherwise 0
    ///
    /// ### Returns
    ///   u8: the value of the cell
    pub fn value(&self) -> u8 {
        return if self.solved() {self.probabilities[0]} else {0}
    }

    /// Check if the cell is solved
    ///
    /// This is the case if the length of the probabilities is 1
    ///
    /// ### Returns
    ///   bool: if the cell is solved
    pub fn solved(&self) -> bool {
        return self.probabilities.len() == 1
    }

    /// Clone the cell
    ///
    /// ### Returns
    ///   Cell: the cloned cell
    pub fn clone(&self) -> Cell {
        return Cell {
            index: self.index,
            probabilities: self.probabilities.clone()
        }
    }
}

impl PartialEq for Cell {
    /// Equality implementation for Cell
    ///
    /// ### Arguments
    ///     other (&Self): other cell to compare
    ///
    /// ### Returns:
    ///     bool: if the cell is the same
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index && self.probabilities == other.probabilities
    }
}

/// ### Strategy
///
/// Contains the name of the strategy and a hashmap with the
/// index of the cell and the probabilities that should be removed from the cell
///
/// The strategy represents a step in the solving process of a sudoku puzzle
///
/// ### Attributes
///    name (String): The name of the strategy
///    remove (HashMap<u8, HashSet<u8>>): The index of the cell and the probabilities that
pub struct Strategy {
    name: String,
    remove: HashMap<u8, HashSet<u8>>,
}


impl Strategy {
    /// Creates a new strategy
    ///
    /// ### Arguments
    ///    name (String): The name of the strategy
    ///    remove (HashMap<u8, HashSet<u8>>): The index of the cell and the probabilities that
    ///     should be removed
    pub fn new(name: String, remove: HashMap<u8, HashSet<u8>>) -> Strategy {
        return Strategy {
            name,
            remove,
        }
    }

    /// Prints the strategy when it has removals
    /// Used for debugging
    ///
    /// The strategy will be printed with the name and the cells that will be removed.
    /// Only prints if there are removals
    pub fn print(&self) {
        if self.remove.is_empty() {
            return
        }

        println!("Strategy: {:?}", self.name);
        for (i, p) in &self.remove {
            println!("For cell {:?}, will remove {:?}", i, p);
        }
    }
}
