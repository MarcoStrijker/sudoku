enum IndexFormulas{}

impl IndexFormulas {
    fn row(i: u8, iter: u8) -> u8 {

        i * 9 + iter
    }

    fn column(i: u8, iter: u8) -> u8 {
        iter * 9 + i
    }

    // Now a static method
    fn block(i: u8, iter: u8) -> u8 {
        27 * (i / 3) + 3 * (i % 3) + (iter / 3) * 9 + (iter % 3)
    }
}

enum BoardIndexFormulas{}

impl BoardIndexFormulas {
    fn row(i: u8, iter: u8) -> u8 {
        (i / 9) * 9 + iter
    }

    fn column(i: u8, iter: u8) -> u8 {
        i % 9 + 9 * iter
    }

    fn block(i: u8, iter: u8) -> u8 {
        // This can now be called as expected
        IndexFormulas::block(i / 3 - i / 9 * 3 + i / 27 * 3, iter)
    }
}


trait  SolveProbabilities {
    fn calulate(board: &mut Board);
}

struct Naked;

impl SolveProbabilities for Naked {
    fn calulate(board: &mut Board) {

    }
}

pub struct Subset {
    pub indices: Vec<u8>,
    pub values: Vec<u8>
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
            indices: (0..9).map(|x| func(i, x)).collect(),
            values: (0..9).map(|x| board.numbers[usize::from(func(i, x))]).collect()
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
        return self.values.contains(value)

    }

    pub fn indices_missing(&self) -> Vec<u8> {
        return self.indices
            .iter()
            .enumerate()
            .filter(|&(ii, _)| *self.values.get(ii).unwrap() == 0)
            .map(|(_, i)| *i)
            .collect();
    }

    pub fn values_missing(&self) -> Vec<u8> {
        return (1..=9)
            .filter(|x| !self.contains(x))
            .collect()
    }

    pub fn has_missing(&self) -> bool {
        return self.values.contains(&0);
    }

    pub fn union(&self, other: &Self) -> Vec<u8> {
        /// Returns all unique solved values that are in self or other
        ///
        /// Args:
        ///     other (Subset): The other subset
        ///
        /// Returns:
        ///     A vector containing the solved values
        let mut union: Vec<u8> = self.values.clone();
        for v in other.values.clone() {
            if v != 0 || union.contains(&v) {
                continue
            }
            union.push(v)
        }

        return union;
    }
}


pub struct Board {
    pub numbers: Vec<u8>,
    pub history: Vec<Vec<u8>>
}


impl Board {

    const ZERO: u8 = 0;

    pub fn from_string(str: &String) -> Board {
        let current_state: Vec<u8> = str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let current_history: Vec<Vec<u8>> =  vec![current_state.clone()];

        return Board {
            numbers: current_state,
            history: current_history,
        };
    }

    pub fn clone(&self) -> Board {
        return Board {
            numbers: self.numbers.clone(),
            history: self.history.clone()
        }
    }

    pub fn to_string(&self) -> String {
        return self.numbers
            .iter()
            .map(|n| n.to_string())
            .collect();
    }

    pub fn print_board(&self) {
        let string: String = self.to_string();

        println!();
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

    pub fn blanks(&self) -> Vec<u8> {
        // Get a vector with the index of the blank cells
        return self.numbers
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value == 0)
            .map(|(index, _)|  index as u8)
            .collect();
    }

    pub fn rollback(&mut self, index: u32) {
        self.numbers = self.history[index as usize].clone();
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
        return self.numbers[index as usize]
    }

    fn set(&mut self, index: u8, solution: u8) {
        // Sets a solution into a cell. Changes the numbers and add the change to the history
        //
        // Args:
        //     index (u8): the index on the board in which you want the solution to be placed
        //     solution (u8): The solution 1-9
        //
        self.numbers[index as usize] = solution;
        self.history.insert(self.history.len(), self.numbers.clone());
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
        return !(self.block_from_index(index).contains(&solution)
            || self.row_from_index(index).contains(&solution)
            || self.column_from_index(index).contains(&solution))
    }

    pub fn solved(&self) -> bool {
        /// Checks if Zero is present in number, if so, the puzzle is not solved
        ///
        /// Returns:
        ///     true is the puzzle is solved, false if not
        ///
        return !self.numbers.contains(&0)
    }

}


impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        return self.history == other.history
    }
}

