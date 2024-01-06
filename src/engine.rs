use super::search::iterative_deepening_search;
use super::util::uci::UciReport;
use async_std::{io::stdin, sync::Arc, task};
use dashmap::DashMap;
use shakmaty::{fen::Fen, uci::Uci, Chess, Position};
use std::{fmt::Result, str::FromStr, sync::OnceLock};

/// Runs the UCI protocol engine, reading from stdin and writing to stdout.
/// Handles commands like `uci`, `isready`, `position`, `go`, etc. and runs
/// the search algorithm when needed.
pub async fn run_uci_engine() -> Result {
    let stdin = stdin();
    let mut message_buffer = String::new();
    let mut position = Chess::new();

    let mut please_stop: Arc<OnceLock<()>> = Arc::new(OnceLock::new());
    let transpositon_table = Arc::new(DashMap::with_capacity(1024 * 1024));

    loop {
        message_buffer.clear();
        stdin.read_line(&mut message_buffer).await.unwrap();
        match UciReport::parse(message_buffer.trim()) {
            UciReport::Uci => {
                println!("id name Axel author Cankyre");
                println!("uciok");
            }
            UciReport::UciNewGame => {
                position = Chess::default();
                transpositon_table.clear();
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
                    iterative_deepening_search(&p, 255, stop, tt, 4);
                });
            }
            UciReport::Stop => if please_stop.set(()).is_ok() {},
            UciReport::GoMoveTime(ms) => {
                please_stop = Arc::new(OnceLock::new());
                let p = position.clone();
                let stop = Arc::clone(&please_stop);
                let tt = Arc::clone(&transpositon_table);

                task::spawn(async move {
                    iterative_deepening_search(&p, 255, stop, tt, 4);
                });

                task::sleep(std::time::Duration::from_millis(ms)).await;

                if please_stop.set(()).is_ok() {}
            }
            UciReport::GoGameTime(game_time) => {
                please_stop = Arc::new(OnceLock::new());
                let p = position.clone();
                let stop = Arc::clone(&please_stop);
                let tt = Arc::clone(&transpositon_table);

                task::spawn(async move {
                    iterative_deepening_search(&p, 255, stop, tt, 2);
                });

                task::sleep(std::time::Duration::from_millis(match position.turn() {
                    shakmaty::Color::White => game_time.wtime / 20 + game_time.winc * 3 / 4,
                    shakmaty::Color::Black => game_time.btime / 20 + game_time.binc * 3 / 4,
                } as u64))
                .await;

                if please_stop.set(()).is_ok() {}
            }
            UciReport::Quit => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
