mod lib;
mod solvers;
mod utils;

use std::iter::zip;
use crate::lib::{Board, Strategy};
use crate::solvers::*;
use crate::utils::brute_force;

fn main() {
    // let start = String::from("400000938032094100095300240370609004529001673604703090957008300003900400240030709");
    let start = String::from("065370002000001370000640800097004028080090001100020940040006700070018050230900060");
    let board = Board::from_string(&start);
    board.print_board();

    let mut super_board: Board = board.clone();
    let mut strategies: Vec<Strategy>;
    let correct: Board = brute_force(board);

    let solvers: Vec<fn(&Board) -> Vec<Strategy>> = vec![
        Naked::get_strategies,
        Hidden::get_strategies,
        Pointing::get_strategies,
    ];

    for _ in 0..6 {
        super_board = LastRemainingCell::get_and_apply_strategies(super_board);
        for s in &solvers {
            strategies = s(&super_board);

            if strategies.is_empty() {
                continue
            }

            super_board = SolveProbabilities::apply_strategies(super_board, strategies);
            break
        }

        for (a, b) in zip(&super_board.cells, &correct.cells) {
            if !a.solved() {
                continue
            }

            if a.probabilities != b.probabilities {
                println!("{:?} is different!!!", a.index);
                panic!()
            }
        }
    }

    super_board.print_board();
}

#[cfg(test)]
mod tests {
    // use crate::utils::brute_force;
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
    fn resolve_probabilities_pointing_one() {
        let mut board = Board::from_string(
            &"017903600000080000900000507072010430000402070064370250701000065000030000005601720"
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[9].contains(&3));
        assert_eq!(true, board.cells[10].contains(&3));
        assert_eq!(true, board.cells[11].contains(&3));
        board = Pointing::get_and_apply_strategies(board);
        assert_eq!(false, board.cells[9].contains(&3));
        assert_eq!(false, board.cells[10].contains(&3));
        assert_eq!(false, board.cells[11].contains(&3));
    }

    #[test]
    fn resolve_probabilities_pointing_two() {
        let mut board = Board::from_string(
            &"930050000200630095856002000003180570005020980080005000000800159508210004000560008"
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[41].contains(&3));
        assert_eq!(true, board.cells[59].contains(&3));
        assert_eq!(true, board.cells[68].contains(&3));
        assert_eq!(true, board.cells[77].contains(&3));
        board = Pointing::get_and_apply_strategies(board);
        assert_eq!(false, board.cells[41].contains(&3));

        assert_eq!(true, board.cells[59].contains(&3));
        assert_eq!(true, board.cells[68].contains(&3));
        assert_eq!(true, board.cells[77].contains(&3));
    }

    #[test]
    fn resolve_probabilities_box_line_reduction_simple() {
        let mut board = Board::from_string(
            &"016007803090800000870001060048000300650009082239000650060900020080002936924600510"
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[3].contains(&2));
        assert_eq!(true, board.cells[4].contains(&2));
        assert_eq!(true, board.cells[13].contains(&2));
        assert_eq!(true, board.cells[21].contains(&2));
        assert_eq!(true, board.cells[22].contains(&2));
        board = BoxLineReduction::get_and_apply_strategies(board);
        assert_eq!(true, board.cells[3].contains(&2));
        assert_eq!(true, board.cells[4].contains(&2));

        assert_eq!(false, board.cells[13].contains(&2));
        assert_eq!(false, board.cells[21].contains(&2));
        assert_eq!(false, board.cells[22].contains(&2));
    }

    #[test]
    fn resolve_probabilities_box_line_reduction_complex() {
        let mut board = Board::from_string(
            &"020943715904000600750000040500480000200000453400352000042000081005004260090208504"
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[28].contains(&6));
        assert_eq!(true, board.cells[29].contains(&6));
        assert_eq!(true, board.cells[37].contains(&6));
        assert_eq!(true, board.cells[38].contains(&6));
        assert_eq!(true, board.cells[46].contains(&6));
        assert_eq!(true, board.cells[47].contains(&6));

        assert_eq!(true, board.cells[54].contains(&3));
        assert_eq!(true, board.cells[63].contains(&3));
        assert_eq!(true, board.cells[64].contains(&3));
        assert_eq!(true, board.cells[72].contains(&3));
        assert_eq!(true, board.cells[74].contains(&3));

        board = BoxLineReduction::get_and_apply_strategies(board);
        assert_eq!(true, board.cells[28].contains(&6));
        assert_eq!(false, board.cells[29].contains(&6));
        assert_eq!(true, board.cells[37].contains(&6));
        assert_eq!(false, board.cells[38].contains(&6));
        assert_eq!(true, board.cells[46].contains(&6));
        assert_eq!(false, board.cells[47].contains(&6));

        assert_eq!(true, board.cells[54].contains(&3));
        assert_eq!(true, board.cells[63].contains(&3));
        assert_eq!(false, board.cells[64].contains(&3));
        assert_eq!(true, board.cells[72].contains(&3));
        assert_eq!(false, board.cells[74].contains(&3));
    }

    #[test]
    fn resolve_probabilities_box_line_reduction_problem() {
        let mut board = Board::from_string(
            &"400070938032894100895306247370609004529001673604703090957008300003960400240035709"
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[79].contains(&6));
        board = BoxLineReduction::get_and_apply_strategies(board);
        assert_eq!(false, board.cells[79].contains(&6));
    }

    #[test]
    fn resolve_probabilities_naked_doubles() {
        let mut board = Board::from_string(
            &"400000938032094100095300240370609004529001673604703090957008300003900400240030709"
                .to_string()
        );

        // We need to resolve all regular probabilities first
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[3].contains(&1));
        assert_eq!(true, board.cells[4].contains(&1));
        assert_eq!(true, board.cells[4].contains(&6));
        assert_eq!(true, board.cells[5].contains(&6));
        assert_eq!(true, board.cells[18].contains(&1));
        assert_eq!(true, board.cells[18].contains(&7));
        assert_eq!(true, board.cells[22].contains(&6));
        assert_eq!(true, board.cells[22].contains(&7));
        board = Naked::get_and_apply_strategies(board);
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
        board = LastRemainingCell::get_and_apply_strategies(board);

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

        board = Naked::get_and_apply_strategies(board);
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
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[1].contains(&1));
        assert_eq!(true, board.cells[1].contains(&5));
        assert_eq!(true, board.cells[2].contains(&5));
        assert_eq!(true, board.cells[11].contains(&5));
        assert_eq!(true, board.cells[11].contains(&6));
        assert_eq!(true, board.cells[11].contains(&8));
        assert_eq!(true, board.cells[20].contains(&6));
        board = Naked::get_and_apply_strategies(board);
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
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[7].contains(&2));
        assert_eq!(true, board.cells[7].contains(&3));
        assert_eq!(true, board.cells[7].contains(&4));
        assert_eq!(true, board.cells[7].contains(&5));
        assert_eq!(true, board.cells[7].contains(&9));
        assert_eq!(true, board.cells[8].contains(&3));
        assert_eq!(true, board.cells[8].contains(&4));
        assert_eq!(true, board.cells[8].contains(&5));
        assert_eq!(true, board.cells[8].contains(&9));
        board = Hidden::get_and_apply_strategies(board);
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
        board = LastRemainingCell::get_and_apply_strategies(board);

        assert_eq!(true, board.cells[3].contains(&4));
        assert_eq!(true, board.cells[3].contains(&7));
        assert_eq!(true, board.cells[3].contains(&8));
        assert_eq!(true, board.cells[6].contains(&4));
        assert_eq!(true, board.cells[6].contains(&9));
        assert_eq!(true, board.cells[8].contains(&4));
        assert_eq!(true, board.cells[8].contains(&7));
        assert_eq!(true, board.cells[8].contains(&8));
        assert_eq!(true, board.cells[8].contains(&9));
        board = Hidden::get_and_apply_strategies(board);
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
        board = LastRemainingCell::get_and_apply_strategies(board);

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
        board = Hidden::get_and_apply_strategies(board);
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