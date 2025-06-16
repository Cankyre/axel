module Search

using Chess
using ThreadPools

export is_search_cancelled, cancel_search, reset_stop_flag, search

include("../Utils/SearchHelper.jl")
include("./Negamax.jl")

const ROOT_POOL = ThreadPools.StaticPool(Threads.nthreads() - 1)

function search(board::Board)
    search(board, 255)
end

function search(board::Board, max_depth::Int)
    reset_stop_flag()
    previous_result = ([], 0.0)

    for depth in 1:max_depth
        if is_search_cancelled()
            break
        end


        result = search_sequential_root(board, depth)

        if is_search_cancelled()
            abs_eval = result[2] * sidetomove(board)
            abs_prev = previous_result[2] * sidetomove(board)
            return abs_eval > abs_prev ? result : previous_result
        end

        if !isnothing(result) && !isempty(result) && isfinite(result[2])
            println("info depth $depth score cp $(round(Int, result[2] * 100)) pv $(join(tostring.(result[1]), " "))")
        end
        previous_result = result
    end

    return previous_result
end

function search_sequential_root(board::Board, depth::Int)
    best_score = -Inf
    best_pv = Move[]

    for move in moves(board)
        u = domove!(board, move)
        pv, score = negamax(board, depth - 1)
        undomove!(board, u)

        if isnothing(score)
            return (best_pv, best_score)
        end

        score = -score
        full_pv = [move; pv...]

        if score > best_score
            best_score = score
            best_pv = full_pv
        end

        yield()
    end

    return (best_pv, best_score)
end

"WARNING: This function is a WIP and should not be used at the moment"
function search_parallel_root(board::Board, depth::Int)
    legalmoves = moves(board)
    results = [([], -Inf) for _ in 1:length(legalmoves)]

    ThreadPools.tforeach(ROOT_POOL, 1:length(legalmoves)) do i
        if is_search_cancelled()
            results[i] = ([], -Inf)
        else
            move = legalmoves[i]
            new_board = domove(board, move)

            pv, score = negamax(new_board, depth - 1)
            score = -score
            results[i] = ([move; pv...], score)
        end
    end

    # Sélection du meilleur
    best_score = -Inf
    best_pv = String[]

    for (pv, score) in results
        if score > best_score
            best_score = score
            best_pv = pv
        end
    end

    return (best_pv, best_score)
end

end
