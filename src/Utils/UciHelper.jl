using Chess

const FEN_STRING_WORD_COUNT = 6

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
        fen_str = join(tokens[i+1:i+FEN_STRING_WORD_COUNT], " ")
        println(fen_str)
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
        i += 6
    else
        println("info string error unknown position format")
        return nothing
    end

    if i <= length(tokens) && tokens[i] == "moves"
        i += 1
        moves = string.(tokens[i:end])
        try
            domoves!(board, movefromstring.(moves)...)
        catch e
            println("info string error illegal move", e)
            return nothing
        end
    end
    
    return board
end