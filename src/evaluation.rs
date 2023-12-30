use shakmaty::{Chess, Color, Position};

mod pesto;
use pesto::*;

pub fn evaluate(board: &Chess) -> i32 {
    let mut phase = 0;

    let color_factor = match board.turn() {
        Color::Black => -1,
        Color::White => 1,
    };

    let mut middlegame_mat_score_colored = (0, 0);
    let mut endgame_mat_score_colored = (0, 0);

    let mut middlegame_pst_score_colored = (0, 0);
    let mut endgame_pst_score_colored = (0, 0);

    for (square, piece) in board.board().clone() {
        phase += game_phase_val(piece.role);

        match piece.color {
            Color::White => {
                let pst = get_pst_score(&piece, &square);
                middlegame_pst_score_colored.1 += pst.0;
                endgame_pst_score_colored.1 += pst.1;

                middlegame_mat_score_colored.1 += middlegame_piece_val(piece.role);
                endgame_mat_score_colored.1 += endgame_piece_val(piece.role);
            }
            Color::Black => {
                let pst = get_pst_score(&piece, &square);
                middlegame_pst_score_colored.0 += pst.0;
                endgame_pst_score_colored.0 += pst.1;

                middlegame_mat_score_colored.0 += middlegame_piece_val(piece.role);
                endgame_mat_score_colored.0 += endgame_piece_val(piece.role);
            }
        }
    }

    let middlegame_pst_score =
        color_factor * (middlegame_pst_score_colored.1 - middlegame_pst_score_colored.0);
    let endgame_pst_score =
        color_factor * (endgame_pst_score_colored.1 - endgame_pst_score_colored.0);

    let middlegame_mat_score =
        color_factor * (middlegame_mat_score_colored.1 - middlegame_mat_score_colored.0);
    let endgame_mat_score =
        color_factor * (endgame_mat_score_colored.1 - endgame_mat_score_colored.0);

    let middlegame_phase = std::cmp::min(phase, 24);
    let endgame_phase = 24 - phase;

    color_factor
        * ((middlegame_mat_score * color_factor + middlegame_pst_score) * middlegame_phase
            + (endgame_mat_score * color_factor + endgame_pst_score) * endgame_phase)
        / 24
}
