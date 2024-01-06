use std::{
    fmt::Display,
    sync::{Arc, OnceLock},
};

use crate::evaluation::Evaluation;
use dashmap::DashMap;
use shakmaty::{zobrist::Zobrist64, CastlingMode, Chess, Move, Position};

use super::transpositon_table::TranspositionTableEntry;

#[derive(Clone)]
pub struct AlphaBetaParams {
    pub depth: u8,
    pub alpha: Evaluation,
    pub beta: Evaluation,
}

impl AlphaBetaParams {
    pub fn new(depth: u8, alpha: Evaluation, beta: Evaluation) -> Self {
        Self { depth, alpha, beta }
    }

    pub fn as_tuple(&self) -> (u8, Evaluation, Evaluation) {
        (self.depth, self.alpha, self.beta)
    }
}

pub struct AlphaBetaInput<'a> {
    pub position: &'a Chess,
    pub stop_hook: &'a Arc<OnceLock<()>>,
    pub transpositon_table: &'a Arc<DashMap<Zobrist64, TranspositionTableEntry>>,
}

type AlphaBetaInputAsTuple<'a> = (
    &'a Chess,
    &'a Arc<OnceLock<()>>,
    &'a Arc<DashMap<Zobrist64, TranspositionTableEntry>>,
);

impl<'a> AlphaBetaInput<'a> {
    pub fn new(
        position: &'a Chess,
        stop_hook: &'a Arc<OnceLock<()>>,
        transpositon_table: &'a Arc<DashMap<Zobrist64, TranspositionTableEntry>>,
    ) -> Self {
        Self {
            position,
            stop_hook,
            transpositon_table,
        }
    }

    pub fn as_tuple(&self) -> AlphaBetaInputAsTuple {
        (self.position, self.stop_hook, self.transpositon_table)
    }
}

pub struct SearchResult {
    pub eval: Evaluation,
    pub pv: Vec<Move>,
    pub nodes: u64,
}

impl SearchResult {
    pub fn new(eval: Evaluation, pv: Vec<Move>, nodes: u64) -> Self {
        Self { eval, pv, nodes }
    }

    pub fn from_evaluation(eval: Evaluation) -> Self {
        Self {
            eval,
            pv: vec![],
            nodes: 1,
        }
    }

    pub fn from_qsearch(qsearch_result: (Evaluation, u64)) -> Self {
        let (eval, nodes) = qsearch_result;
        Self::new(eval, vec![], nodes)
    }

    // pub fn default() -> Self {
    //     Self::from_evaluation(Evaluation::Min)
    // }
}

impl Clone for SearchResult {
    fn clone(&self) -> Self {
        Self {
            eval: self.eval,
            pv: self.pv.clone(),
            nodes: self.nodes,
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pv_str = self
            .pv
            .iter()
            .map(|m| m.to_uci(CastlingMode::Standard).to_string())
            .rev()
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "score {} nodes {} pv {}", self.eval, self.nodes, pv_str)
    }
}

pub trait ChessExt {
    fn with(&self, m: &Move) -> Chess;
}

impl ChessExt for Chess {
    fn with(&self, m: &Move) -> Chess {
        let mut p = self.clone();
        p.play_unchecked(m);
        p
    }
}
