const COEFFICIENTS_LENGTH = 5 + 12 * 64

const SQUARE_TO_INT = Dict{Square,Int}(
    SQ_A1 => 1,
    SQ_A2 => 2,
    SQ_A3 => 3,
    SQ_A4 => 4,
    SQ_A5 => 5,
    SQ_A6 => 6,
    SQ_A7 => 7,
    SQ_A8 => 8,
    SQ_B1 => 9,
    SQ_B2 => 10,
    SQ_B3 => 11,
    SQ_B4 => 12,
    SQ_B5 => 13,
    SQ_B6 => 14,
    SQ_B7 => 15,
    SQ_B8 => 16,
    SQ_C1 => 17,
    SQ_C2 => 18,
    SQ_C3 => 19,
    SQ_C4 => 20,
    SQ_C5 => 21,
    SQ_C6 => 22,
    SQ_C7 => 23,
    SQ_C8 => 24,
    SQ_D1 => 25,
    SQ_D2 => 26,
    SQ_D3 => 27,
    SQ_D4 => 28,
    SQ_D5 => 29,
    SQ_D6 => 30,
    SQ_D7 => 31,
    SQ_D8 => 32,
    SQ_E1 => 33,
    SQ_E2 => 34,
    SQ_E3 => 35,
    SQ_E4 => 36,
    SQ_E5 => 37,
    SQ_E6 => 38,
    SQ_E7 => 39,
    SQ_E8 => 40,
    SQ_F1 => 41,
    SQ_F2 => 42,
    SQ_F3 => 43,
    SQ_F4 => 44,
    SQ_F5 => 45,
    SQ_F6 => 46,
    SQ_F7 => 47,
    SQ_F8 => 48,
    SQ_G1 => 49,
    SQ_G2 => 50,
    SQ_G3 => 51,
    SQ_G4 => 52,
    SQ_G5 => 53,
    SQ_G6 => 54,
    SQ_G7 => 55,
    SQ_G8 => 56,
    SQ_H1 => 57,
    SQ_H2 => 58,
    SQ_H3 => 59,
    SQ_H4 => 60,
    SQ_H5 => 61,
    SQ_H6 => 62,
    SQ_H7 => 63,
    SQ_H8 => 64,
    SQ_NONE => -1,
)
struct EvaluationParameters
    pawn_value::Float64
    knight_value::Float64
    bishop_value::Float64
    rook_value::Float64
    queen_value::Float64
    pst_pawn_mg::Vector{Float64}
    pst_pawn_eg::Vector{Float64}
    pst_knight_mg::Vector{Float64}
    pst_knight_eg::Vector{Float64}
    pst_bishop_mg::Vector{Float64}
    pst_bishop_eg::Vector{Float64}
    pst_rook_mg::Vector{Float64}
    pst_rook_eg::Vector{Float64}
    pst_queen_mg::Vector{Float64}
    pst_queen_eg::Vector{Float64}
    pst_king_mg::Vector{Float64}
    pst_king_eg::Vector{Float64}
end

const _COEFFS = Ref{Union{Nothing,EvaluationParameters}}(nothing)

function get_coefficients()
    if isnothing(_COEFFS[])
        _COEFFS[] = load_coefficients(coefficient_array)
    end
    return _COEFFS[]
end

"""
    load_coefficients(path::String)

Axel relies on a linear evaluation function. The coefficient values are determined
by a set of predefined weights that are loaded from a file, trained using a machine learning algorithm.
"""
function load_coefficients(values::Vector{Float64})
    # The order of coefficients is assumed to match the struct fields
    if length(values) < COEFFICIENTS_LENGTH
        println("info string error not enough coefficients ($(length(values)) < $COEFFICIENTS_LENGTH)")
    end

    function extract_pst(start_idx)
        return values[start_idx:start_idx+63]
    end


    idx = 6
    return EvaluationParameters(
        values[1],  # pawn_value
        values[2],  # knight_value
        values[3],  # bishop_value
        values[4],  # rook_value
        values[5],  # queen_value
        extract_pst(idx),            # pst_pawn_mg
        extract_pst(idx + 64),       # pst_pawn_eg
        extract_pst(idx + 64 * 2),   # pst_knight_mg
        extract_pst(idx + 64 * 3),   # pst_knight_eg
        extract_pst(idx + 64 * 4),   # pst_bishop_mg
        extract_pst(idx + 64 * 5),   # pst_bishop_eg
        extract_pst(idx + 64 * 6),   # pst_rook_mg
        extract_pst(idx + 64 * 7),   # pst_rook_eg
        extract_pst(idx + 64 * 8),   # pst_queen_mg
        extract_pst(idx + 64 * 9),   # pst_queen_eg
        extract_pst(idx + 64 * 10),  # pst_king_mg
        extract_pst(idx + 64 * 11)   # pst_king_eg
    )
end

function Int(square::Square)
    return SQUARE_TO_INT[square]
end

function extract_mg_pst(piece::Piece, square::Square, coeffs::EvaluationParameters)
    s = Int(square)
    if piece == PIECE_WP
        return coeffs.pst_pawn_mg[s]
    elseif piece == PIECE_BP
        return -coeffs.pst_pawn_mg[s⊻56]
    elseif piece == PIECE_WN
        return coeffs.pst_knight_mg[s]
    elseif piece == PIECE_BN
        return -coeffs.pst_knight_mg[s⊻56]
    elseif piece == PIECE_WB
        return coeffs.pst_bishop_mg[s]
    elseif piece == PIECE_BB
        return -coeffs.pst_bishop_mg[s⊻56]
    elseif piece == PIECE_WR
        return coeffs.pst_rook_mg[s]
    elseif piece == PIECE_BR
        return -coeffs.pst_rook_mg[s⊻56]
    elseif piece == PIECE_WQ
        return coeffs.pst_queen_mg[s]
    elseif piece == PIECE_BQ
        return -coeffs.pst_queen_mg[s⊻56]
    elseif piece == PIECE_WK
        return coeffs.pst_king_mg[s]
    elseif piece == PIECE_BK
        return -coeffs.pst_king_mg[s⊻56]
    elseif piece == EMPTY
        return 0
    end
end

function extract_eg_pst(piece::Piece, square::Square, coeffs::EvaluationParameters)
    s = Int(square)
    if piece == PIECE_WP
        return coeffs.pst_pawn_eg[s]
    elseif piece == PIECE_BP
        return -coeffs.pst_pawn_eg[s⊻56]
    elseif piece == PIECE_WN
        return coeffs.pst_knight_eg[s]
    elseif piece == PIECE_BN
        return -coeffs.pst_knight_eg[s⊻56]
    elseif piece == PIECE_WB
        return coeffs.pst_bishop_eg[s]
    elseif piece == PIECE_BB
        return -coeffs.pst_bishop_eg[s⊻56]
    elseif piece == PIECE_WR
        return coeffs.pst_rook_eg[s]
    elseif piece == PIECE_BR
        return -coeffs.pst_rook_eg[s⊻56]
    elseif piece == PIECE_WQ
        return coeffs.pst_queen_eg[s]
    elseif piece == PIECE_BQ
        return -coeffs.pst_queen_eg[s⊻56]
    elseif piece == PIECE_WK
        return coeffs.pst_king_eg[s]
    elseif piece == PIECE_BK
        return -coeffs.pst_king_eg[s⊻56]
    elseif piece == EMPTY
        return 0
    end
end

function extract_game_phase_inc(piece::Piece)
    # Returns the game phase increment for a given piece type.
    if piece == PIECE_WP
        return 0
    elseif piece == PIECE_BP
        return 0
    elseif piece == PIECE_WN
        return 1
    elseif piece == PIECE_BN
        return 1
    elseif piece == PIECE_WB
        return 1
    elseif piece == PIECE_BB
        return 1
    elseif piece == PIECE_WR
        return 2
    elseif piece == PIECE_BR
        return 2
    elseif piece == PIECE_WQ
        return 4
    elseif piece == PIECE_BQ
        return 4
    elseif piece == PIECE_WK
        return 0
    elseif piece == PIECE_BK
        return 0
    end
end
