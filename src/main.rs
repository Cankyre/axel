use async_std::task;
mod engine;
mod evaluation;
mod search;
mod util;

use engine::run_uci_engine;

fn main() {
    task::block_on(run_uci_engine()).unwrap()
}
