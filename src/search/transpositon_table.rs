pub use dashmap::DashMap;

use super::{super::evaluation::Evaluation, util::SearchResult};
use shakmaty::Move;

#[derive(Clone)]
pub struct TranspositionTableEntry {
    pub eval: Evaluation,
    pub depth: u8,
    pub pv: Vec<Move>,
    pub nodes: u64,
}

impl TranspositionTableEntry {
    pub fn new(eval: Evaluation, depth: u8, pv: Vec<Move>, nodes: u64) -> Self {
        Self {
            eval,
            depth,
            pv,
            nodes,
        }
    }

    pub fn from_search_result(result: &SearchResult, depth: u8) -> Self {
        Self::new(result.eval, depth, result.pv.clone(), result.nodes)
    }
}
