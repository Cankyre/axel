use shakmaty::Move;

fn score_move(move_: &Move) -> i32 {
    if move_.is_capture() {
        return -12 * move_.capture().unwrap() as i32 - move_.role() as i32;
    }

    0
}

pub fn order_moves(moves: &[Move]) -> Vec<Move> {
    let mut moves = moves.to_vec();

    moves.sort_by_key(score_move);

    moves
}
