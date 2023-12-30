use super::search_evaluation::Evaluation;
use shakmaty::Move;

pub struct SearchResult {
    pub eval: Evaluation,
    pub pv: Vec<Move>,
}
