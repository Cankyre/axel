use std::{fmt::Result, str::FromStr, sync::OnceLock};

use async_std::{
    io::stdin,
    sync::{Arc, Mutex},
    task,
};

use uci::UciReport;

use shakmaty::{fen::Fen, uci::Uci, Chess, Position};

use crate::search::iterative_deepening_search;
mod evaluation;
mod search;
mod uci;

use search::transpositon_table::TranspositionTable;

async fn run_uci_engine() -> Result {
    let stdin = stdin();
    let mut message_buffer = String::new();
    let mut position = Chess::new();

    let mut please_stop: Arc<OnceLock<()>> = Arc::new(OnceLock::new());
    let transpositon_table = Arc::new(Mutex::new(TranspositionTable::new()));

    loop {
        message_buffer.clear();
        stdin.read_line(&mut message_buffer).await.unwrap();
        match UciReport::parse(message_buffer.trim()) {
            UciReport::Uci => {
                println!("uciok");
            }
            UciReport::UciNewGame => {
                position = Chess::default();
                transpositon_table.lock().await.clear();
            }
            UciReport::IsReady => println!("readyok"),
            UciReport::Position(fen, moves) => {
                position = fen
                    .parse::<Fen>()
                    .unwrap()
                    .into_position(shakmaty::CastlingMode::Standard)
                    .unwrap();

                for m in moves {
                    let parsed = Uci::from_str(&m);
                    if let Ok(parsed) = parsed {
                        position.play_unchecked(&parsed.to_move(&position).unwrap());
                    }
                }
            }
            UciReport::GoInfinite => {
                please_stop = Arc::new(OnceLock::new());
                let p = position.clone();
                let stop = Arc::clone(&please_stop);
                let tt = Arc::clone(&transpositon_table);

                task::spawn(async move {
                    iterative_deepening_search(&p, 10, stop, tt).await;
                });
            }
            UciReport::Stop => if please_stop.set(()).is_ok() {},
            UciReport::GoMoveTime(ms) => {
                please_stop = Arc::new(OnceLock::new());
                let p = position.clone();
                let stop = Arc::clone(&please_stop);
                let tt = Arc::clone(&transpositon_table);

                task::spawn(async move {
                    iterative_deepening_search(&p, 256, stop, tt).await;
                });

                task::sleep(std::time::Duration::from_millis(ms)).await;

                if please_stop.set(()).is_ok() {}
            }
            _ => {}
        }
    }
}

fn main() {
    task::block_on(run_uci_engine()).unwrap();
}
