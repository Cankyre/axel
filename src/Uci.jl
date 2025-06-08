module UCI

using Chess

const STARTPOS_FEN = "rn1qkbnr/pppb1ppp/8/3pp3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 1"

mutable struct UciEngine
    board::Chess.Board
    display_info::Bool
end

function UciEngine()
    return UciEngine(fromfen(STARTPOS_FEN), true)
end

function uci_loop()
    engine = UciEngine()

    while true
        line = readline(stdin)
        tokens = split(line)
        cmd = tokens[1]
        args = tokens[2:end]

        if cmd == "uci"
            println("id name Axel")
            println("id author Cankyre")
            println("uciok")
        elseif cmd == "isready"
            println("readyok")
        elseif cmd == "ucinewgame"
            engine = UciEngine()
        elseif cmd == "position"
            if args[1] == "startpos"
                engine.board = fromfen(STARTPOS_FEN)
            else
                engine.board = fromfen(join(args, " "))
            end
        elseif cmd == "go"
            # TODO
            continue
        elseif cmd == "quit"
            break
        end
    end
end

end
