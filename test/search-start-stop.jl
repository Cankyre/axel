using Test
using Axel.Search

@testset "Search cancellation handling" begin
    @test !is_search_cancelled()
    @test cancel_search()
    @test is_search_cancelled()
    @test !reset_stop_flag()
    @test !is_search_cancelled() 
end