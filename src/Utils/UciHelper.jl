using Chess

const FEN_STRING_WORD_COUNT = 6

"""Turns a tokens array (corresponding to a `position` command's args) into a valid board.
NOTE: It is the caller's responsibility to ensure the moves sent are legal."""
function parse_position(tokens::Vector{SubString{String}})
    i = 1
    board = nothing
    if tokens[i] == "startpos"
        board = startboard()
        i += 1
    elseif tokens[i] == "fen"
        if i + FEN_STRING_WORD_COUNT > length(tokens)
            println("info string error invalid FEN too short")
            return nothing
        end
        fen_str = join(tokens[(i+1):(i+FEN_STRING_WORD_COUNT)], " ")
        try
            board = fromfen(fen_str)
            if isnothing(board)
                println("info string error invalid FEN")
                return nothing
            end
        catch
            println("info string error invalid FEN")
            return nothing
        end
        i += 7
    else
        println("info string error unknown position format")
        return nothing
    end
    if i <= length(tokens) && tokens[i] == "moves"
        i += 1
        moves_str = string.(tokens[i:end])
        try
            domoves!(board, movefromstring.(moves_str)...)
        catch e
            println("info string error illegal move", e)
            return nothing
        end
    end

    return board
end

"""Chess.jl's `fen(Chess::Board)` function often outputs the wrong en-passant square (epsq)
This bodge fixes it. It should not be used for performance-intensive tasks"""
function fen_fixed_epsq(b::Chess.Board)
    fen_str = fen(b)
    last_move = lastmove(b)
    last_piece = pieceon(b, to(last_move))

    last_piece_was_pawn = last_piece == PIECE_WP || last_piece == PIECE_BP
    from_second_rank = rank(from(last_move)) == RANK_2 || rank(from(last_move)) == RANK_7
    to_fourth_rank = rank(to(last_move)) == RANK_4 || rank(to(last_move)) == RANK_5

    if last_piece_was_pawn && from_second_rank && to_fourth_rank
        eprk = rank(from(last_move)) == RANK_2 ? RANK_3 : RANK_6
        epsq = Square(file(from(last_move)), eprk)

        fen_str = fen_str[1:(end-1)] * tostring(epsq)
    else
        return fen_str
    end
end
