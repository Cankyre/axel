using Test
using Axel.UCI 
using Chess

@testset "`position` command parsing" begin
    tokens = split("startpos")
    @test fen_fixed_epsq(parse_position(tokens)) == fen(startboard())

    tokens = split("startpos moves e2e4 e7e5")
    expected_fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6"
    b = parse_position(tokens)
    @test fen_fixed_epsq(parse_position(tokens)) == expected_fen

    fen_str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    expected_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq -"
    tokens = split("fen $fen_str")
    @test fen_fixed_epsq(parse_position(tokens)) == expected_fen

    tokens = split("fen $fen_str moves c7c5")
    expected_fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6"
    @test fen_fixed_epsq(parse_position(tokens)) == expected_fen

    tokens = split("fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b")
    @test parse_position(tokens) === nothing

    tokens = split("banana")
    @test parse_position(tokens) === nothing
end
