use shakmaty::{Chess, Position};

use super::Evaluation;

pub fn get_obvious_evaluation(position: &Chess) -> Option<Evaluation> {
    if position.is_checkmate() {
        return Some(Evaluation::Mate(0));
    }

    if position.is_stalemate() || position.is_insufficient_material() {
        return Some(Evaluation::Centipawns(0));
    }

    None
}
