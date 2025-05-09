default: test

## test: run test
test:
	cargo test

## ci-test: test ci workflow locally using act.
ci-test: 
	act -W .github/workflows/test-and-build.yml

## check: linter and format
check: 
	cargo clippy
	cargo fmt