use hashbrown::HashMap;

use super::search_evaluation::Evaluation;
use shakmaty::{zobrist::Zobrist64, Move};

#[derive(Clone)]
pub struct TranspositionTableEntry {
    pub eval: Evaluation,
    pub depth: u16,
    pub pv: Vec<Move>,
}

impl TranspositionTableEntry {
    pub fn new(eval: Evaluation, depth: u16, pv: Vec<Move>) -> Self {
        Self { eval, depth, pv }
    }
}

pub struct TranspositionTable {
    entries: HashMap<Zobrist64, TranspositionTableEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        let size = 512 * 1024 * 1024;
        let entry_size = std::mem::size_of::<TranspositionTableEntry>();
        Self {
            entries: HashMap::with_capacity(size / entry_size),
        }
    }

    pub fn get(&self, hash: Zobrist64) -> Option<&TranspositionTableEntry> {
        self.entries.get(&hash)
    }

    pub fn insert(&mut self, hash: Zobrist64, entry: TranspositionTableEntry) {
        self.entries.insert(hash, entry);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
