module Evaluation

using Chess

include("CoefficientArray.jl")
include("../Utils/EvalHelper.jl")

export get_coefficients, evaluate

function evaluate(board::Board)
    coeffs = get_coefficients()

    wp_count = length(squares(pawns(board, WHITE)))
    bp_count = length(squares(pawns(board, BLACK)))
    wn_count = length(squares(knights(board, WHITE)))
    bn_count = length(squares(knights(board, BLACK)))
    wb_count = length(squares(bishops(board, WHITE)))
    bb_count = length(squares(bishops(board, BLACK)))
    wr_count = length(squares(rooks(board, WHITE)))
    br_count = length(squares(rooks(board, BLACK)))
    wq_count = length(squares(queens(board, WHITE)))
    bq_count = length(squares(queens(board, BLACK)))

    pawn_score = (wp_count - bp_count) * coeffs.pawn_value
    knight_score = (wn_count - bn_count) * coeffs.knight_value
    bishop_score = (wb_count - bb_count) * coeffs.bishop_value
    rook_score = (wr_count - br_count) * coeffs.rook_value
    queen_score = (wq_count - bq_count) * coeffs.queen_value

    material_score = pawn_score + knight_score + bishop_score + rook_score + queen_score

    # PST
    gamephase_value = 0
    mg_pst_score = 0.0
    eg_pst_score = 0.0

    for square in squares(pieces(board, WHITE) ∪ pieces(board, BLACK))
        println(square)
        println(pieceon(board, square))
        mg_pst_score += extract_mg_pst(pieceon(board, square), square, coeffs)
        println(pieceon(board, square))
        eg_pst_score += extract_eg_pst(pieceon(board, square), square, coeffs)
        println(pieceon(board, square))
        gamephase_value += extract_game_phase_inc(pieceon(board, square))
        println(square, " done")
    end

    gamephase_value = max(24, gamephase_value)
    pst_score = mg_pst_score * gamephase_value + eg_pst_score * (24 - gamephase_value)

    total_score = material_score + pst_score

    sign = sidetomove(board) == WHITE ? 1.0 : -1.0
    println("Total Score: ", total_score)
    return total_score * sign
end

end
