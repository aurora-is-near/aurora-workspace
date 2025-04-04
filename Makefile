CLIPPY_RULES = -D warnings

MOCK_DIR = res
ENGINE_DIR = mock_engine
ETH_CONNECTOR_DIR = mock_eth_connector
ENGINE_MOCK_DIR = ${MOCK_DIR}/${ENGINE_DIR}
ETH_CONNECTOR_MOCK_DIR = ${MOCK_DIR}/${ETH_CONNECTOR_DIR}
MOCK_CARGO_BUILD = RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

ENGINE_MOCK_FILE = ${ENGINE_MOCK_DIR}/target/wasm32-unknown-unknown/release/mock_engine.wasm
ETH_CONNECTOR_MOCK_FILE = ${ETH_CONNECTOR_MOCK_DIR}/target/wasm32-unknown-unknown/release/mock_eth_connector.wasm

clean_engine_mock:
	@cd ${ENGINE_MOCK_DIR} && cargo clean

clean_eth_connector_mock:
	@cd ${ETH_CONNECTOR_MOCK_DIR} && cargo clean

clean_workspace:
	@rm -rf bin && cargo clean

clippy-mock-engine:
	@cd ${ENGINE_MOCK_DIR} && \
	cargo clippy -- ${CLIPPY_RULES}

clippy-mock-eth-connector:
	@cd ${ETH_CONNECTOR_MOCK_DIR} && \
	cargo clippy -- ${CLIPPY_RULES}

clippy-lib:
	@cargo clippy -- ${CLIPPY_RULES}

clippy-test:
	@cargo clippy --tests -- ${CLIPPY_RULES}

check-fmt:
	@cargo fmt -- --check
	@cd ${ENGINE_MOCK_DIR} && cargo fmt -- --check
	@cd ${ETH_CONNECTOR_MOCK_DIR} && cargo fmt -- --check

fmt:
	@cargo fmt --all

build-mock-engine:
	@cd ${ENGINE_MOCK_DIR} && ${MOCK_CARGO_BUILD}

build-mock-eth-connector:
	@cd ${ETH_CONNECTOR_MOCK_DIR} && ${MOCK_CARGO_BUILD}

test-engine:
	@cargo test --package aurora-workspace-engine -- --test-threads 10 --nocapture

test-eth-connector:
	@cargo test --package aurora-workspace-eth-connector -- --test-threads 10 --nocapture

check: check-fmt clippy

clippy: clippy-lib clippy-test clippy-mock-engine clippy-mock-eth-connector

clean: clean_engine_mock clean_eth_connector_mock clean_workspace

test-flow: test-engine test-eth-connector

test: test-flow
