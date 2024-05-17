mod lib;
mod solvers;
mod utils;


use crate::lib::Board;

use crate::solvers::*;
use crate::utils::brute_force;


fn main() {
    let start = String::from("400000938032094100095300240370609004529001673604703090957008300003900400240030709");
    // let start = String::from("065370002000001370000640800097004028080090001100020940040006700070018050230900060");
    let mut board = Board::from_string(&start);
    board.print_board();

    let new_board = LastRemainingCell::calculate(&mut board);
    println!("LastRemainingCell");
    new_board.print_board();

    let newest_board = Naked::calculate(&mut board);
    println!("Naked");
    newest_board.print_board();

    let new_newest = Hidden::calculate(&mut board);
    println!("Hidden");
    new_newest.print_board();

    let new_new_newer = Pointing::calculate(&mut board);
    println!("Pointing");
    new_new_newer.print_board();

    brute_force(Board::from_string(&start)).print_board();

}

#[cfg(test)]
mod tests {
    use crate::utils::brute_force;
    use super::*;

    #[test]
    fn solve_via_brute_force() {
        let mut board: Board;
        let puzzles = utils::import_puzzles_from_file();

        for puzzle in puzzles {
            board = Board::from_string(&puzzle[0]);

            for _ in 0..100 {
                board = brute_force(board);
            }

            assert_eq!(board.to_string(), puzzle[1]);
        }
    }

    #[test]
    fn solve_via_direct_solver() {
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