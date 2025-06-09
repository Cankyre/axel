module UCI

using Chess

const STARTPOS_FEN = "rn1qkbnr/pppb1ppp/8/3pp3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 1"

include("Utils/UciHelper.jl")

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

        if isempty(line)
            continue
        end

        tokens = split(line)

        if isempty(tokens)
            continue
        end
        
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
            b = parse_position(args)
            if isnothing(b)
                println("info string error position not registered")
            else
                engine.board = b
            end
        elseif cmd == "go"
            # TODO
            println("info string error not implemented")
        elseif cmd == "quit"
            break
        elseif cmd in ["debug", "setoption", "register", "stop", "ponderhit"]
            println("info string error not implemented")
        else
            println("info string error invalid command") 
        end
    end
end

export UciEngine, uci_loop, parse_position, fen_fixed_epsq

end
