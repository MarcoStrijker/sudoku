mod lib;
mod solvers;
mod utils;


use crate::lib::Board;

use crate::solvers::{DirectSolvers, LastRemainingCellBlock};



fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let mut board: Board;
        let puzzles = utils::import_puzzles_from_file();

        for puzzle in puzzles {
            board = Board::from_string(&puzzle[0]);

            for _ in 0..100 {
                board = LastRemainingCellBlock.solve(board);
            }

            assert_eq!(board.to_string(), puzzle[1]);
        }
    }
}