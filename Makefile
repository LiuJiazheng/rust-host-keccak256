pull: ## Pull all the recursive submodule
	@git submodule update --init --recursive
clean:
	@rm -rf output
	@rm -rf param
	@rm -rf *.json
	@rm -rf sol/contracts/AggregatorVerifierStep*.sol
	@rm -rf sol/contracts/AggregatorConfig.sol
	@rm -rf pkg
	@rm -rf target
	@rm -rf zkWasm/target
	@rm -rf zkWasm-host-circuits/target
	@rm -rf continuation-batcher/target

build:
	@wasm-pack build --release
	@cd zkWasm && cargo build --features cuda --release
	@cd zkWasm-host-circuits && cargo build --features cuda --release
	@cd continuation-batcher && cargo build --features cuda --release

