"Handles UCI and anything related to communication with other programs."
module UCI

using Chess
using ..Search

const STARTPOS_FEN = "rn1qkbnr/pppb1ppp/8/3pp3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 1"

include("Utils/UciHelper.jl")

"""Handles the state of an ongoing UCI communication"""
mutable struct UciState
    board::Chess.Board
    display_info::Bool
end

function UciState()
    return UciState(fromfen(STARTPOS_FEN), true)
end

"""Initiates and manages a UCI loop
- Supported: `uci`, `isready`, `ucinewgame`, `position`, `quit`.
If a command is not yet implemented, will send an `info string error`"""
function uci_loop()
    search_task = nothing
    engine = UciState()

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
            engine = UciState()
        elseif cmd == "position"
            b = parse_position(args)
            if isnothing(b)
                println("info string error position not registered")
            else
                engine.board = b
            end
        elseif cmd == "go"
            if !isnothing(search_task)
                println("info string error search already running")
            end
            search_task = @async begin
                bestmove = Search.search(engine.board)
                println("bestmove $(bestmove[0][1])")
                search_task = nothing
            end
        elseif cmd == "stop"
            println("info string received stop")
            cancel_search()
        elseif cmd == "quit"
            break
        elseif cmd in ["debug", "setoption", "register", "stop", "ponderhit"]
            println("info string error not implemented")
        else
            println("info string error invalid command")
        end
    end
end

export UciState, uci_loop, parse_position, fen_fixed_epsq

end
