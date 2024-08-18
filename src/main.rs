mod lib;
mod solvers;
mod utils;


use crate::lib::Board;

use crate::solvers::*;
use crate::utils::brute_force;

// fn main() {
//     let start = String::from("400000938032094100095300240370609004529001673604703090957008300003900400240030709");
//     // let start = String::from("065370002000001370000640800097004028080090001100020940040006700070018050230900060");
//     let mut board = Board::from_string(&start);
//     board.print_board();
//
//     let mut super_board: Board = board.clone();
//
//     for _ in 0..5 {
//         super_board = LastRemainingCell::calculate(&mut super_board);
//         super_board = Naked::calculate(&mut super_board);
//         super_board = Hidden::calculate(&mut super_board);
//         super_board = Pointing::calculate(&mut super_board);
//     }
//
//     super_board.print_board();
//
//     brute_force(Board::from_string(&start)).print_board();
//
// }

#[cfg(test)]
mod tests {
    use crate::utils::brute_force;
    use super::*;

    // #[test]
    // fn solve_via_brute_force() {
    //     let mut board: Board;
    //     let puzzles = utils::import_puzzles_from_file();
    //
    //     for puzzle in puzzles {
    //         board = Board::from_string(&puzzle[0]);
    //
    //         for _ in 0..100 {
    //             board = brute_force(board);
    //         }
    //
    //         assert_eq!(board.to_string(), puzzle[1]);
    //     }
    // }

    #[test]
    fn resolve_probabilities_pointing() {
        let mut board = Board::from_string(
            &"017903600000080000900000507072010430000402070064370250701000065000030000005601720"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[9].contains(&3));
        assert_eq!(true, board.cells[10].contains(&3));
        assert_eq!(true, board.cells[11].contains(&3));
        board = Pointing::calculate(&mut board);
        assert_eq!(false, board.cells[9].contains(&3));
        assert_eq!(false, board.cells[10].contains(&3));
        assert_eq!(false, board.cells[11].contains(&3));
    }

    #[test]
    fn resolve_probabilities_naked_doubles() {
        let mut board = Board::from_string(
            &"400000938032094100095300240370609004529001673604703090957008300003900400240030709"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[3].contains(&1));
        assert_eq!(true, board.cells[4].contains(&1));
        assert_eq!(true, board.cells[4].contains(&6));
        assert_eq!(true, board.cells[5].contains(&6));
        assert_eq!(true, board.cells[18].contains(&1));
        assert_eq!(true, board.cells[18].contains(&7));
        assert_eq!(true, board.cells[22].contains(&6));
        assert_eq!(true, board.cells[22].contains(&7));
        board = Naked::calculate(&mut board);
        assert_eq!(false, board.cells[3].contains(&1));
        assert_eq!(false, board.cells[4].contains(&1));
        assert_eq!(false, board.cells[4].contains(&6));
        assert_eq!(false, board.cells[5].contains(&6));
        assert_eq!(false, board.cells[18].contains(&1));
        assert_eq!(false, board.cells[22].contains(&6));
        assert_eq!(false, board.cells[22].contains(&7));
    }

    #[test]
    fn resolve_probabilities_naked_triples() {
        let mut board = Board::from_string(
            &"070408029002000004854020007008374200020000000003261700000093612200000403130642070"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[36].contains(&5));
        assert_eq!(true, board.cells[36].contains(&9));
        assert_eq!(true, board.cells[38].contains(&5));
        assert_eq!(true, board.cells[38].contains(&9));
        assert_eq!(true, board.cells[42].contains(&5));
        assert_eq!(true, board.cells[42].contains(&8));
        assert_eq!(true, board.cells[42].contains(&9));
        assert_eq!(true, board.cells[43].contains(&5));
        assert_eq!(true, board.cells[43].contains(&8));
        assert_eq!(true, board.cells[43].contains(&9));

        board = Naked::calculate(&mut board);
        assert_eq!(false, board.cells[36].contains(&5));
        assert_eq!(false, board.cells[36].contains(&9));
        assert_eq!(false, board.cells[38].contains(&5));
        assert_eq!(false, board.cells[38].contains(&9));
        assert_eq!(false, board.cells[42].contains(&5));
        assert_eq!(false, board.cells[42].contains(&8));
        assert_eq!(false, board.cells[42].contains(&9));
        assert_eq!(false, board.cells[43].contains(&5));
        assert_eq!(false, board.cells[43].contains(&8));
        assert_eq!(false, board.cells[43].contains(&9));
    }

    #[test]
    fn resolve_probabilities_naked_quads() {
        let mut board = Board::from_string(
            &"000030086000020040090078520371856294900142375400397618200703859039205467700904132"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[1].contains(&1));
        assert_eq!(true, board.cells[1].contains(&5));
        assert_eq!(true, board.cells[2].contains(&5));
        assert_eq!(true, board.cells[11].contains(&5));
        assert_eq!(true, board.cells[11].contains(&6));
        assert_eq!(true, board.cells[11].contains(&8));
        assert_eq!(true, board.cells[20].contains(&6));
        board = Naked::calculate(&mut board);
        assert_eq!(false, board.cells[1].contains(&1));
        assert_eq!(false, board.cells[1].contains(&5));
        assert_eq!(false, board.cells[2].contains(&5));
        assert_eq!(false, board.cells[11].contains(&5));
        assert_eq!(false, board.cells[11].contains(&6));
        assert_eq!(false, board.cells[11].contains(&8));
        assert_eq!(false, board.cells[20].contains(&6));


    }


    #[test]
    fn resolve_probabilities_hidden_doubles() {
        let mut board = Board::from_string(
            &"000000000904607000076804100309701080708000301051308702007502610005403208000000000"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[7].contains(&2));
        assert_eq!(true, board.cells[7].contains(&3));
        assert_eq!(true, board.cells[7].contains(&4));
        assert_eq!(true, board.cells[7].contains(&5));
        assert_eq!(true, board.cells[7].contains(&9));
        assert_eq!(true, board.cells[8].contains(&3));
        assert_eq!(true, board.cells[8].contains(&4));
        assert_eq!(true, board.cells[8].contains(&5));
        assert_eq!(true, board.cells[8].contains(&9));
        board = Hidden::calculate(&mut board);
        assert_eq!(false, board.cells[7].contains(&2));
        assert_eq!(false, board.cells[7].contains(&3));
        assert_eq!(false, board.cells[7].contains(&4));
        assert_eq!(false, board.cells[7].contains(&5));
        assert_eq!(false, board.cells[7].contains(&9));
        assert_eq!(false, board.cells[8].contains(&3));
        assert_eq!(false, board.cells[8].contains(&4));
        assert_eq!(false, board.cells[8].contains(&5));
        assert_eq!(false, board.cells[8].contains(&9));

        assert_eq!(true, board.cells[7].contains(&6));
        assert_eq!(true, board.cells[7].contains(&7));
        assert_eq!(true, board.cells[8].contains(&6));
        assert_eq!(true, board.cells[8].contains(&7));
    }

    #[test]
    fn resolve_probabilities_hidden_triples() {
        let mut board = Board::from_string(
            &"000001030231090000065003100678924300103050006000136700009360570006019843300000000"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[3].contains(&4));
        assert_eq!(true, board.cells[3].contains(&7));
        assert_eq!(true, board.cells[3].contains(&8));
        assert_eq!(true, board.cells[6].contains(&4));
        assert_eq!(true, board.cells[6].contains(&9));
        assert_eq!(true, board.cells[8].contains(&4));
        assert_eq!(true, board.cells[8].contains(&7));
        assert_eq!(true, board.cells[8].contains(&8));
        assert_eq!(true, board.cells[8].contains(&9));
        board = Hidden::calculate(&mut board);
        assert_eq!(false, board.cells[3].contains(&4));
        assert_eq!(false, board.cells[3].contains(&7));
        assert_eq!(false, board.cells[3].contains(&8));
        assert_eq!(false, board.cells[6].contains(&4));
        assert_eq!(false, board.cells[6].contains(&9));
        assert_eq!(false, board.cells[8].contains(&4));
        assert_eq!(false, board.cells[8].contains(&7));
        assert_eq!(false, board.cells[8].contains(&8));
        assert_eq!(false, board.cells[8].contains(&9));

        assert_eq!(true, board.cells[6].contains(&2));
        assert_eq!(true, board.cells[6].contains(&6));
        assert_eq!(true, board.cells[8].contains(&2));
        assert_eq!(true, board.cells[8].contains(&5));
    }

    #[test]
    fn resolve_probabilities_hidden_quads() {
        let mut board = Board::from_string(
            &"901500046425090081860010020502000000019000460600000002196040253200060817000001694"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::calculate(&mut board);

        assert_eq!(true, board.cells[30].contains(&3));
        assert_eq!(true, board.cells[30].contains(&7));
        assert_eq!(true, board.cells[30].contains(&8));
        assert_eq!(true, board.cells[32].contains(&3));
        assert_eq!(true, board.cells[32].contains(&7));
        assert_eq!(true, board.cells[32].contains(&8));
        assert_eq!(true, board.cells[48].contains(&3));
        assert_eq!(true, board.cells[48].contains(&7));
        assert_eq!(true, board.cells[48].contains(&8));
        assert_eq!(true, board.cells[50].contains(&3));
        assert_eq!(true, board.cells[50].contains(&5));
        assert_eq!(true, board.cells[50].contains(&7));
        assert_eq!(true, board.cells[50].contains(&8));
        board = Hidden::calculate(&mut board);
        assert_eq!(false, board.cells[30].contains(&3));
        assert_eq!(false, board.cells[30].contains(&7));
        assert_eq!(false, board.cells[30].contains(&8));
        assert_eq!(false, board.cells[32].contains(&3));
        assert_eq!(false, board.cells[32].contains(&7));
        assert_eq!(false, board.cells[32].contains(&8));
        assert_eq!(false, board.cells[48].contains(&3));
        assert_eq!(false, board.cells[48].contains(&7));
        assert_eq!(false, board.cells[48].contains(&8));
        assert_eq!(false, board.cells[50].contains(&3));
        assert_eq!(false, board.cells[50].contains(&5));
        assert_eq!(false, board.cells[50].contains(&7));
        assert_eq!(false, board.cells[50].contains(&8));

        assert_eq!(true, board.cells[30].contains(&1));
        assert_eq!(true, board.cells[30].contains(&4));
        assert_eq!(true, board.cells[30].contains(&6));
        assert_eq!(true, board.cells[30].contains(&9));

        assert_eq!(true, board.cells[32].contains(&4));
        assert_eq!(true, board.cells[32].contains(&6));
        assert_eq!(true, board.cells[32].contains(&9));

        assert_eq!(true, board.cells[48].contains(&1));
        assert_eq!(true, board.cells[48].contains(&4));
        assert_eq!(true, board.cells[48].contains(&9));

        assert_eq!(true, board.cells[50].contains(&4));
        assert_eq!(true, board.cells[50].contains(&9));
    }
}