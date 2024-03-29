struct Board {
    pub numbers: Vec<u8>,
    pub history: Vec<Vec<u8>>
}


impl Board {

    const ZERO: u8 = 0;

    fn new() -> Board {
        return Board {
            numbers: vec![0; 80],
            history: Vec::new()
        }
    }

    fn from_string(str: &String) -> Board {
        let current_state: Vec<u8> = str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let current_history: Vec<Vec<u8>> =  vec![current_state.clone()];
        return Board {
            numbers: current_state,
            history: current_history
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

    pub fn rollback(&mut self, index: u32) {
        self.numbers = self.history[index as usize].clone();
        // self.history.truncate(index as usize + 1);
    }

    pub fn get(&self, index: u8) -> u8 {
        return self.numbers[index as usize]
    }

    pub fn set(&mut self, index: u8, solution: u8) -> bool {
        if index > 80 {
            panic!("An set on a index higher than 80")
        }

        if !self.validate(index, solution) {
            return false;
        }

        self.numbers[index as usize] = solution;
        self.history.insert(self.history.len(), self.numbers.clone());
        return true
    }

    pub fn set_in_row(&mut self, n: u8, index: u8, solution: u8) -> bool {
        let real_index: u8 = n * 9 + index;
        return self.set(real_index, solution)
    }

    pub fn set_in_column(&mut self, n: u8, index: u8, solution: u8) -> bool {
        let real_index: u8 = index * 9 + n;
        return self.set(real_index, solution)
    }

    pub fn set_in_quadrant(&mut self, n: u8, index: u8, solution: u8) -> bool {
        let real_index: u8 = 27 * (n / 3) + 3 * (n % 3) + (index / 3) * 9 + (index % 3);
        return self.set(real_index, solution)
    }

    pub fn column(&self, n: u8) -> Vec<u8> {
        return (0..9).map(|i| self.numbers[usize::from(i * 9 + n)]).collect();
    }

    pub fn row(&self, n: u8) -> Vec<u8> {
        return (0..9).map(|i| self.numbers[usize::from(n * 9 + i)]).collect();
    }

    pub fn quadrant(&self, n: u8) -> Vec<u8> {
        return (0..9).map(|i| self.numbers[usize::from(27 * (n / 3) + 3 * (n % 3) + (i / 3) * 9 + (i % 3))]).collect();
    }

    pub fn row_from_index(&self, i: u8) -> Vec<u8> {
        return self.row(i / 9)
    }

    pub fn column_from_index(&self, i: u8) -> Vec<u8> {
        return self.column(i % 9)
    }

    pub fn quadrant_from_index(&self, i: u8) -> Vec<u8> {
        return self.quadrant(i / 3 - i / 9 * 3 + i / 27 * 3)
    }

    pub fn validate(&self, index: u8, solution: u8) -> bool {
        if self.quadrant_from_index(index).contains(&solution) {
            return false
        }
        if self.row_from_index(index).contains(&solution) {
            return false
        }
        if self.column_from_index(index).contains(&solution) {
            return false
        }
        return true
    }

    pub fn solved(&self) -> bool {
        return !self.numbers.contains(&Self::ZERO)
    }

}


trait SolvingStrategy {
    fn solve(&self, board: Board) -> Board;
}

struct SimpleLogic;

impl SolvingStrategy for SimpleLogic {
    fn solve(&self, mut board: Board) -> Board {
        let mut num: u8;
        let mut index: u8;
        let mut valid: bool;
        let mut collection: Vec<u8>;

        // TODO: Optimize
        for i in 0..8 {
            collection = board.row(i);
            if collection.iter().filter(|&&x| x == 0).count() == 1 {
                num = 45 - collection.iter().map(|&x| x as u8).sum::<u8>();
                index = collection.iter().position(|&x| x == 0).unwrap() as u8;
                valid = board.set_in_row(i, index, num as u8);
                if !valid {
                    continue
                }
                return board;
            }
        }

        for i in 0..8 {
            collection = board.column(i);
            if collection.iter().filter(|&&x| x == 0).count() == 1 {
                num = 45 - collection.iter().map(|&x| x as u8).sum::<u8>();
                index = collection.iter().position(|&x| x == 0).unwrap() as u8;
                valid = board.set_in_column(i, index, num as u8);
                if !valid {
                    continue
                }
                return board;
            }
        }

        for i in 0..8 {
            collection = board.quadrant(i);
            if collection.iter().filter(|&&x| x == 0).count() == 1 {
                num = 45 - collection.iter().map(|&x| x as u8).sum::<u8>();
                index = collection.iter().position(|&x| x == 0).unwrap() as u8;
                valid = board.set_in_quadrant(i, index, num as u8);
                if !valid {
                    continue
                }
                return board;
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
    let blanks: Vec<u8> = board.numbers
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value == 0)
        .map(|(index, _)|  index as u8)
        .collect();

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

        // Add new solutiom
        count += 1;
        solve_history.insert(current_index as usize, count);
        current_index += 1;
        addition = 1;
    }

    return board
}


fn main() {
    let start = String::from("695127304138459672724836915851264739273981546946573821317692458489715263562348197");
    let end = String::from("695127304138459672724836915851264739273981546946573821317692458489715263562348197");
    let b = Board::from_string(&start);
    b.print_board();
    let new_b = SimpleLogic.solve(b);
    new_b.print_board();
}
