module Axel

using Chess

include("Evaluation/Evaluation.jl")
include("Search/Search.jl")
include("Uci.jl")

export UCI, Search, Evaluation

end # module Axel
