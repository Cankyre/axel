use std::{sync::OnceLock, thread};

use async_std::sync::Arc;
use shakmaty::{
    zobrist::{Zobrist64, ZobristHash},
    CastlingMode, Chess, Move, Position,
};

use super::evaluation::{
    evaluate,
    obvious::get_obvious_evaluation,
    Evaluation::{self, *},
};

mod move_ordering;
pub mod transpositon_table;
mod util;

use move_ordering::order_moves;
use transpositon_table::*;
use util::*;

fn quiescence_search(position: &Chess, alpha: Evaluation, beta: Evaluation) -> (Evaluation, u64) {
    let mut alpha = alpha;

    let standing_pat = Evaluation::Centipawns(evaluate(position));

    if let Some(e) = get_obvious_evaluation(position) {
        return (e, 1);
    }

    if standing_pat >= beta {
        return (beta, 1);
    }

    if alpha < standing_pat {
        alpha = standing_pat;
    }

    let captures = order_moves(&position.capture_moves());

    let mut nodes = 1;
    for m in captures {
        let p = position.with(&m);

        let res = quiescence_search(&p, beta.to_deeper(), alpha.to_deeper());
        let eval = Evaluation::from_deeper(&res.0);
        nodes += res.1;

        if beta <= eval {
            return (beta, nodes);
        }

        if eval > alpha {
            alpha = eval
        }
    }

    (alpha, nodes)
}

pub fn alpha_beta_search(params: AlphaBetaParams, input: AlphaBetaInput) -> Option<SearchResult> {
    let (depth, alpha, beta) = params.as_tuple();
    let (position, please_stop, transpositon_table) = input.as_tuple();

    if please_stop.get().is_some() {
        return None;
    }

    if let Some(eval) = get_obvious_evaluation(position) {
        return Some(SearchResult::from_evaluation(eval));
    }

    if depth == 0 {
        return Some(SearchResult::from_qsearch(quiescence_search(
            position, alpha, beta,
        )));
    }

    let table_hit = transpositon_table.get(&position.zobrist_hash(shakmaty::EnPassantMode::Legal));
    if let Some(entry) = table_hit {
        if entry.depth >= depth {
            return Some(SearchResult::new(entry.eval, entry.pv.clone(), 1));
        }
    }

    let mut nodes = 1;
    let mut alpha = alpha;
    let mut best_pv: Vec<Move> = vec![];

    let moves = order_moves(&position.legal_moves());

    for m in moves {
        nodes += 1;

        let p = position.with(&m);

        let mut move_result = match alpha_beta_search(
            AlphaBetaParams::new(depth - 1, beta.to_deeper(), alpha.to_deeper()),
            AlphaBetaInput::new(&p, &please_stop.clone(), &transpositon_table.clone()),
        ) {
            Some(r) => r,
            None => return None,
        };

        nodes += move_result.nodes;
        let eval = Evaluation::from_deeper(&move_result.eval);

        if beta <= eval {
            transpositon_table.insert(
                position.zobrist_hash(shakmaty::EnPassantMode::Legal),
                TranspositionTableEntry::new(beta, depth, move_result.pv.clone(), nodes),
            );
            move_result.eval = beta;
            move_result.pv.push(m);
            return Some(move_result);
        }

        if alpha < eval {
            alpha = eval;
            best_pv = move_result.pv;
            best_pv.push(m);
        }
    }

    let best_results = SearchResult::new(alpha, best_pv, nodes);

    transpositon_table.insert(
        position.zobrist_hash(shakmaty::EnPassantMode::Legal),
        TranspositionTableEntry::from_search_result(&best_results, depth),
    );

    Some(best_results)
}

pub fn iterative_deepening_search(
    position: &Chess,
    max_depth: u8,
    please_stop: Arc<OnceLock<()>>,
    transpositon_table: Arc<DashMap<Zobrist64, TranspositionTableEntry>>,
    num_threads: u8,
) {
    let start_time = std::time::Instant::now();
    let mut best_results = None;

    for depth in 1..=max_depth {
        let mut visited_nodes = 0;

        let mut threads = vec![];
        for _ in 0..num_threads {
            let please_stop = Arc::clone(&please_stop);
            let transpositon_table = Arc::clone(&transpositon_table);
            let position = position.clone();
            threads.push(thread::spawn(move || {
                alpha_beta_search(
                    AlphaBetaParams::new(depth, Min, Max),
                    AlphaBetaInput::new(&position, &please_stop, &transpositon_table),
                )
            }));
        }

        let mut results = vec![];
        for t in threads {
            let r = match t.join() {
                Ok(r) => r,
                Err(_) => None,
            };

            if let Some(result) = r {
                results.push(result);
            }
        }

        let mut res = None;
        for r in results {
            if res.is_none() {
                res = Some(r);
                continue;
            }

            if r.eval > res.as_ref().unwrap().eval {
                res = Some(r);
            }
        }

        if let Some(result) = res {
            visited_nodes += result.nodes;
            best_results = Some(result.clone());

            if result.eval == Mate(depth as i32) {
                break;
            }
        }

        if please_stop.get().is_none() {
            println!(
                "info {} depth {} time {} nps {} hashfull {}",
                best_results.as_ref().unwrap(),
                depth,
                start_time.elapsed().as_millis(),
                visited_nodes / (start_time.elapsed().as_secs() + 1),
                transpositon_table.len() * 1000 / transpositon_table.capacity()
            )
        }
    }

    println!(
        "bestmove {}",
        best_results
            .as_ref()
            .unwrap()
            .pv
            .last()
            .unwrap()
            .to_uci(CastlingMode::Standard)
    );
}
