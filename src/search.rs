use std::sync::OnceLock;

use async_std::sync::{Arc, Mutex};
use shakmaty::{zobrist::ZobristHash, CastlingMode, Chess, Move, Position};

use async_recursion::async_recursion;

use super::evaluation::evaluate;

mod move_ordering;
mod search_evaluation;
mod search_result;
pub mod transpositon_table;

use search_evaluation::Evaluation::{self, *};
use search_result::SearchResult;
use transpositon_table::*;

#[async_recursion]
async fn quiescence_search(
    position: &Chess,
    alpha: Evaluation,
    beta: Evaluation,
    visited_nodes: &mut u64,
) -> Evaluation {
    let mut alpha = alpha;

    let standing_pat = Evaluation::LowerBound(evaluate(position));

    if position.is_checkmate() {
        return Mate(0);
    }

    if standing_pat >= beta {
        return beta;
    }

    if alpha < standing_pat {
        alpha = standing_pat;
    }

    let captures = move_ordering::order_moves(&position.capture_moves());

    *visited_nodes += captures.len() as u64;
    for m in captures {
        let mut p = position.clone();

        p.play_unchecked(&m);
        let eval = Evaluation::from_deeper(
            &quiescence_search(
                &p,
                Evaluation::from_deeper(&beta),
                Evaluation::from_deeper(&alpha),
                visited_nodes,
            )
            .await,
        );

        if beta <= eval {
            return beta;
        }

        if eval > alpha {
            alpha = eval
        }
    }

    match alpha {
        Mate(a) => Mate(a),
        LowerBound(a) | UpperBound(a) => Exact(a),
        _ => alpha,
    }
}

pub async fn add_to_transposition_table(
    position: &Chess,
    eval: &Evaluation,
    depth: u16,
    pv: &[Move],
    transpositon_table: &Arc<Mutex<TranspositionTable>>,
) {
    let mut guard = transpositon_table.lock().await;
    guard.insert(
        position.zobrist_hash(shakmaty::EnPassantMode::Legal),
        TranspositionTableEntry::new(eval.clone(), depth, Vec::from(pv)),
    );
    drop(guard);
}

pub async fn get_from_transposition_table<'a>(
    position: &Chess,
    transpositon_table: &Arc<Mutex<TranspositionTable>>,
    depth_left: u16,
) -> Option<TranspositionTableEntry> {
    let guard = transpositon_table.lock().await;
    if let Some(entry) = guard.get(position.zobrist_hash(shakmaty::EnPassantMode::Legal)) {
        if entry.depth >= depth_left {
            return Some(entry.clone());
        }
    }

    None
}

#[async_recursion]
pub async fn alpha_beta_search(
    position: &Chess,
    alpha: Evaluation,
    beta: Evaluation,
    depth_left: u16,
    visited_nodes: &mut u64,
    please_stop: &Arc<OnceLock<()>>,
    transpositon_table: &Arc<Mutex<TranspositionTable>>,
) -> Option<SearchResult> {
    if depth_left == 0 {
        return Some(SearchResult {
            eval: quiescence_search(position, alpha, beta, visited_nodes).await,
            pv: vec![],
        });
    }

    {
        let guard = transpositon_table.lock().await;
        if let Some(entry) = guard.get(position.zobrist_hash(shakmaty::EnPassantMode::Legal)) {
            if entry.depth >= depth_left {
                return Some(SearchResult {
                    eval: entry.eval.clone(),
                    pv: entry.pv.clone(),
                });
            }
        }
    }

    let mut best_eval = alpha.clone();
    let mut best_pv = vec![];
    let moves = move_ordering::order_moves(&position.legal_moves());

    *visited_nodes += moves.len() as u64;

    for m in moves {
        if please_stop.get().is_some() {
            return None;
        }

        let mut p = position.clone();
        p.play_unchecked(&m);

        if p.is_checkmate() {
            return Some(SearchResult {
                eval: Mate(1),
                pv: vec![m],
            });
        }

        if p.is_stalemate() || p.is_insufficient_material() {
            return Some(SearchResult {
                eval: Exact(0),
                pv: vec![m],
            });
        }

        let eval;
        let mut pv;

        let tt_result = get_from_transposition_table(&p, transpositon_table, depth_left).await;

        if let Some(hit) = tt_result {
            eval = hit.eval;
            pv = hit.pv;
        } else {
            let move_result = alpha_beta_search(
                &p,
                Evaluation::from_deeper(&beta),
                Evaluation::from_deeper(&best_eval),
                depth_left - 1,
                visited_nodes,
                please_stop,
                transpositon_table,
            )
            .await;

            move_result.as_ref()?;

            eval = Evaluation::from_deeper(&move_result.as_ref().unwrap().eval);
            pv = move_result.unwrap().pv;
        }

        if beta <= eval {
            pv.push(m);
            add_to_transposition_table(position, &beta, depth_left, &pv, transpositon_table).await;
            return Some(SearchResult { eval: beta, pv });
        }

        if best_eval < eval {
            best_eval = eval;

            pv.push(m);
            best_pv = pv;
        }
    }

    add_to_transposition_table(
        position,
        &best_eval,
        depth_left,
        &best_pv,
        transpositon_table,
    )
    .await;

    Some(SearchResult {
        eval: best_eval,
        pv: best_pv,
    })
}

fn format_info(eval: &Evaluation, depth: u16, nodes: u64, pv: &[Move], time: u128) -> String {
    let pv = pv
        .iter()
        .map(|m| m.to_uci(CastlingMode::Standard).to_string())
        .rev()
        .collect::<Vec<String>>()
        .join(" ");

    format!(
        "info score {} depth {} nodes {} time {} pv {} ",
        eval_to_string(eval),
        depth,
        nodes,
        time,
        pv
    )
}

fn eval_to_string(eval: &Evaluation) -> String {
    match eval {
        Exact(v) => format!("cp {}", v),
        LowerBound(v) => format!("lowerbound {}", v),
        UpperBound(v) => format!("upperbound {}", v),
        Mate(v) => format!(
            "mate {}",
            match v {
                0 => 0,
                v => v.signum() * (v.abs() + 1) / 2,
            }
        ),
        Max => format!("cp {}", i32::MAX),
        Min => format!("cp {}", i32::MIN),
    }
}

pub async fn iterative_deepening_search(
    position: &Chess,
    max_depth: u16,
    please_stop: Arc<OnceLock<()>>,
    transpositon_table: Arc<Mutex<TranspositionTable>>,
) -> Option<SearchResult> {
    let start = std::time::Instant::now();
    let mut best_results = None;

    for depth in 1..=max_depth {
        let mut visited_nodes = 0;
        let res = alpha_beta_search(
            position,
            Min,
            Max,
            depth,
            &mut visited_nodes,
            &please_stop,
            &transpositon_table,
        )
        .await;

        if let Some(result) = res {
            best_results = Some(result);
        }

        if please_stop.get().is_none() {
            println!(
                "{}",
                format_info(
                    &best_results.as_ref().unwrap().eval,
                    depth,
                    visited_nodes,
                    &best_results.as_ref().unwrap().pv,
                    start.elapsed().as_millis()
                )
            );
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

    best_results
}
