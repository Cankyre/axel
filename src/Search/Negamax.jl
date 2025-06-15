using Chess

function negamax(board::Board, depth::Int)
    return negamax(board, depth, -Inf, Inf)
end

function negamax(
    board::Board,
    depth::Int,
    α::Float64,
    β::Float64,
)
    if depth == 0
        # TODO Replace with actual evaluation function
        return ([], rand(-10.0:10.0:0.1, 1)[1])
    end

    if is_search_cancelled()
        return ([], nothing)
    end

    best_value = -Inf
    best_pv = []
    movelist = moves(board)

    for move in movelist
        undoinfo = domove!(board, move)
        (pv, value) = negamax(board, depth - 1, -β, -α)
        undomove!(board, undoinfo)

        if isnothing(value)
            return (best_pv, best_value)
        end

        value = -value

        if value > best_value
            best_value = value
            best_pv = [move, pv...]
        end

        α = max(α, value)
        if α >= β
            break
        end
    end

    return (best_pv, best_value)
end
