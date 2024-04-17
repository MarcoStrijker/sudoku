enum IndexFormulas{}

impl IndexFormulas {
    fn row(i: u8, iter: u8) -> u8 { i * 9 + iter }
    fn column(i: u8, iter: u8) -> u8 {
        iter * 9 + i
    }
    fn block(i: u8, iter: u8) -> u8 { 27 * (i / 3) + 3 * (i % 3) + (iter / 3) * 9 + (iter % 3) }
}

enum BoardIndexFormulas{}

impl BoardIndexFormulas {
    fn row(i: u8, iter: u8) -> u8 {
        (i / 9) * 9 + iter
    }
    fn column(i: u8, iter: u8) -> u8 {
        i % 9 + 9 * iter
    }
    fn block(i: u8, iter: u8) -> u8 { IndexFormulas::block(i / 3 - i / 9 * 3 + i / 27 * 3, iter) }
}


pub struct Subset {
    pub indices: Vec<u8>,
    pub cells: Vec<Cell>
}


impl Subset {
    fn from_board(board: &Board, i: u8, func: &dyn Fn(u8, u8) -> u8) -> Subset {
        /// Initializes a subset of Cells from a board instance based on the requested
        /// index and passed function. Eligible functions can be found in IndexFormulas
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
        return Subset {
            indices: (0..9)
                .map(|x| func(i, x))
                .collect(),
            cells: (0..9)
                .map(|x| board.cells[usize::from(func(i, x))].clone())
                .collect()
        }
    }

    pub fn contains(&self, value: &u8) -> bool {
        /// Wrapper for Vector.contains. Looks into the values of the
        /// subset if it contains the passed value
        ///
        /// Args:
        ///     value (u8): The value that should be checked
        ///
        /// Return:
        ///     True if the value is in self.values
        ///
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

    pub fn values_solved(&self) -> Vec<u8> {
        /// Get the values that are solved in Self
        ///
        /// Returns:
        ///     The solved values
        return self.cells
            .iter()
            .filter(|c| c.solved())
            .map(|c| c.value())
            .collect()
    }

    pub fn values_missing(&self) -> Vec<u8> {
        return self.cells
            .iter()
            .filter(|c| !c.solved())
            .map(|c| c.value())
            .collect()
    }

    pub fn has_missing(&self) -> bool {
        return self.cells
            .iter()
            .any(|c| !c.solved())
    }

    pub fn union(&self, other: &Self) -> Vec<u8> {
        /// Returns all unique solved values that are in self or other
        ///
        /// Args:
        ///     other (Subset): The other subset
        ///
        /// Returns:
        ///     A vector containing the solved values
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
    pub cells: Vec<Cell>,
    pub history: Vec<Vec<Cell>>
}


impl Board {

    pub fn from_string(str: &String) -> Board {
        let current_state: Vec<Cell> = str
            .chars()
            .enumerate()
            .map(|(i, char)| Cell::from_number(i, char.to_digit(10).unwrap()))
            .collect();
        let current_history: Vec<Vec<Cell>> =  vec![current_state.clone()];

        return Board {
            cells: current_state,
            history: current_history
        };
    }

    pub fn clone(&self) -> Board {
        return Board {
            cells: self.cells.clone(),
            history: self.history.clone(),
        }
    }

    // pub fn convert_probabilities_to_solution(&mut self) {
    //     for (i, cell) in self.cells.iter_mut().enumerate() {
    //         if *number != 0 {
    //             continue
    //         }
    //
    //         if self.probabilities[i].len() != 1 {
    //             continue
    //         }
    //
    //         *number = self.probabilities[i][0]
    //     }
    // }

    pub fn to_string(&self) -> String {
        return self.cells
            .iter()
            .map(|c| c.value().to_string())
            .collect();
    }

    pub fn print_board(&self) {
        let string: String = self.to_string();
        let percentage_completed: f32 = self.cells
            .iter()
            .filter(|c| c.solved())
            .count() as f32 / 81 as f32 * 100 as f32;

        println!();
        println!("{:?} - {:?}%", self.uncertainty(), percentage_completed);
        for (i, s) in string.chars().enumerate() {
            // After each row, print a new line
            if i != 0 && i % 9 == 0 {
                println!();
            }

            // After three rows, print a row separation
            if i != 0 && i % 27 == 0 {
                println!(" —  —  —   —  —  —   —  —  — ");
            }

            // Print column separators
            if i != 0 && i % 3 == 0 && i % 9 != 0 {
                print!("|")
            }
            print!(" {} ", s);
        }
        println!()
    }

    pub fn uncertainty(&self) -> usize {
        /// Returns the sum of the number of probabilities
        /// Acts as measure towards solving a puzzle
        ///
        /// Returns:
        ///     Number of probabilities (usize)
        ///
        return self.cells
            .iter()
            .map(|c| c.probabilities.len())
            .sum::<usize>()
    }

    pub fn blanks(&self) -> Vec<u8> {
        /// Get a vector with the index of the blank cells
        return self.cells
            .iter()
            .filter(|(c)| !c.solved())
            .map(|c| c.index)
            .collect();
    }

    pub fn rollback(&mut self, index: u32) {
        self.cells = self.history[index as usize].clone();
        // self.history.truncate(index as usize + 1);
    }

    pub fn get(&self, index: u8) -> u8 {
        /// Get the value of a cell, naturally cannot exceed 80
        ///
        /// Args:
        ///     index (u8): the index of the cell
        ///
        /// Returns:
        ///     The value of the cell (u8)
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
        self.history.insert(self.history.len(), self.cells.clone());
    }

    pub fn try_set(&mut self, index: u8, solution: u8) -> bool {
        /// Try to set a solution into a cell
        ///
        /// Args:
        ///     index (u8): the index on the board in which you want the solution to be placed
        ///     solution (u8): The solution 1-9
        ///
        /// Returns:
        ///     true if the set is valid, false if not (bool)
        ///
        if index > 80 {
            panic!("You've tried setting a number with a to high index. Not allowed")
        }

        // Check if the set is according to the sudoku rules
        // Check for conflicts in ros, column, and block
        if !self.validate(index, solution) {
            return false;
        }

        // Set the solution
        self.set(index, solution);
        return true
    }

    pub fn row(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &IndexFormulas::row)
    }

    pub fn column(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &IndexFormulas::column)
    }

    pub fn block(&self, i: u8) -> Subset {
        return Subset::from_board(self, i, &IndexFormulas::block)
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

    pub fn solved(&self) -> bool {
        /// Checks if the board is solved
        ///
        /// Returns:
        ///     true is the puzzle is solved, false if not
        ///
        return self.cells
            .iter()
            .all(|c| c.solved())
    }

}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        return self.history == other.history
    }
}

#[derive(Clone)]
pub struct Cell {
    pub index: u8,
    pub probabilities: Vec<u8>
}

impl Cell {
    fn from_number(i: usize, number: u32) -> Cell {
        return Cell {
            index: i as u8,
            probabilities: if number == 0 {vec![1,2,3,4,5,6,7,8,9]} else {vec![number as u8]}
        }
    }

    pub fn row(&self) -> u8 {
        return self.index / 9
    }

    pub fn column(&self) -> u8 {
        return self.index % 9
    }

    pub fn block(&self) -> u8 {
        return self.index / 3 - self.index / 9 * 3 + self.index / 27 * 3
    }

    pub fn contains(&self, value: &u8) -> bool {
        return self.probabilities.contains(value)
    }

    pub fn set(&mut self, value: &u8) {
        self.probabilities = vec![*value]
    }

    pub fn remove(&mut self, value: u8) {
        self.probabilities = self.probabilities
            .clone()
            .iter()
            .filter(|p| p == &&value)
            .map(|p| *p)
            .collect()
    }

    pub fn value(&self) -> u8 {
        return if self.solved() {self.probabilities[0]} else {0}
    }

    pub fn solved(&self) -> bool {
        return self.probabilities.len() == 1
    }

    pub fn clone(&self) -> Cell {
        return Cell {
            index: self.index.clone(),
            probabilities: self.probabilities.clone()
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index && self.probabilities == other.probabilities
    }
}
