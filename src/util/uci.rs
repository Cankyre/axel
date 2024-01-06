// Input will be turned into a report, which wil be sent to the engine. The
// main engine thread will react accordingly.

static FEN_STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use super::gametime::GameTime;

#[derive(PartialEq, Clone)]
pub enum UciReport {
    // Uci commands
    Uci,
    UciNewGame,
    IsReady,
    // SetOption(EngineOptionName),
    Position(String, Vec<String>),
    GoInfinite,
    GoDepth(i8),
    GoMoveTime(u64),
    GoNodes(usize),
    GoGameTime(GameTime),
    Stop,
    Quit,

    // Empty or unknown command.
    Unknown,
}

// Public functions
impl UciReport {
    fn parse_go(cmd: &str) -> UciReport {
        enum Tokens {
            Nothing,
            Depth,
            Nodes,
            MoveTime,
            WTime,
            BTime,
            WInc,
            BInc,
            MovesToGo,
        }

        let parts: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
        let mut report = UciReport::Unknown;
        let mut token = Tokens::Nothing;
        let mut game_time = GameTime::new(0, 0, 0, 0);

        for p in parts {
            match p {
                t if t == "go" => report = UciReport::GoInfinite,
                t if t == "infinite" => break, // Already Infinite; nothing more to do.
                t if t == "depth" => token = Tokens::Depth,
                t if t == "movetime" => token = Tokens::MoveTime,
                t if t == "nodes" => token = Tokens::Nodes,
                t if t == "wtime" => token = Tokens::WTime,
                t if t == "btime" => token = Tokens::BTime,
                t if t == "winc" => token = Tokens::WInc,
                t if t == "binc" => token = Tokens::BInc,
                t if t == "movestogo" => token = Tokens::MovesToGo,
                _ => match token {
                    Tokens::Nothing => (),
                    Tokens::Depth => {
                        let depth = p.parse::<i8>().unwrap_or(1);
                        report = UciReport::GoDepth(depth);
                        break; // break for-loop: nothing more to do.
                    }
                    Tokens::MoveTime => {
                        let milliseconds = p.parse::<u64>().unwrap_or(1000);
                        report = UciReport::GoMoveTime(milliseconds);
                        break; // break for-loop: nothing more to do.
                    }
                    Tokens::Nodes => {
                        let nodes = p.parse::<usize>().unwrap_or(1);
                        report = UciReport::GoNodes(nodes);
                        break; // break for-loop: nothing more to do.
                    }
                    Tokens::WTime => game_time.wtime = p.parse::<u128>().unwrap_or(0),
                    Tokens::BTime => game_time.btime = p.parse::<u128>().unwrap_or(0),
                    Tokens::WInc => game_time.winc = p.parse::<u128>().unwrap_or(0),
                    Tokens::BInc => game_time.binc = p.parse::<u128>().unwrap_or(0),
                    // Tokens::MovesToGo => {
                    //     game_time.moves_to_go = if let Ok(x) = p.parse::<usize>() {
                    //         Some(x)
                    //     } else {
                    //         None
                    //     }
                    // }
                    _ => (),
                }, // end match token
            } // end match p
        } // end for

        // If we are still in the default "go infinite" mode, we must
        // switch to GameTime mode if at least one parameter of "go wtime
        // btime winc binc" was set to something else but 0.
        let is_default_mode = report == UciReport::GoInfinite;
        let has_time = game_time.wtime > 0 || game_time.btime > 0;
        let has_inc = game_time.winc > 0 || game_time.binc > 0;
        let is_game_time = has_time || has_inc;
        if is_default_mode && is_game_time {
            report = UciReport::GoGameTime(game_time);
        }

        report
    } // end parse_go()

    fn parse_position(cmd: &str) -> UciReport {
        enum Tokens {
            Nothing,
            Fen,
            Moves,
        }

        let parts: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
        let mut fen = String::from("");
        let mut moves: Vec<String> = Vec::new();
        let mut skip_fen = false;
        let mut token = Tokens::Nothing;

        for p in parts {
            match p {
                t if t == "position" => (), // Skip. We know we're parsing "position".
                t if t == "startpos" => skip_fen = true, // "fen" is now invalidated.
                t if t == "fen" && !skip_fen => token = Tokens::Fen,
                t if t == "moves" => token = Tokens::Moves,
                _ => match token {
                    Tokens::Nothing => (),
                    Tokens::Fen => {
                        fen.push_str(&p[..]);
                        fen.push(' ');
                    }
                    Tokens::Moves => moves.push(p),
                },
            }
        }

        // No FEN part in the command. Use the start position.
        if fen.is_empty() {
            fen = String::from(FEN_STARTPOS);
        }

        UciReport::Position(fen.trim().to_string(), moves)
    }

    // Parse a command
    pub fn parse(cmd: &str) -> UciReport {
        match cmd {
            // UCI commands
            "uci" => UciReport::Uci,
            "ucinewgame" => UciReport::UciNewGame,
            "isready" => UciReport::IsReady,
            "stop" => UciReport::Stop,
            "quit" | "exit" => UciReport::Quit,
            // cmd if cmd.starts_with("setoption") => Uci::parse_setoption(&cmd),
            cmd if cmd.starts_with("position") => UciReport::parse_position(cmd),
            cmd if cmd.starts_with("go") => UciReport::parse_go(cmd),

            // Everything else is ignored.
            _ => UciReport::Unknown,
        }
    }
}
