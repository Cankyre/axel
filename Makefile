.PHONY: install test

install:
	@echo "Install dependencies"
	julia --project=. -e 'using Pkg; Pkg.instantiate()'

test:
	@echo "Run tests"
	julia --project=. -e 'using Pkg; Pkg.test()'
format:
	@echo "Format code"
	julia --project=. -e 'using JuliaFormatter; format(\".\", verbose=true)'
