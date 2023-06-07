CLIPPY_RULES = -D warnings

MOCK_DIR = res
ENGINE_DIR = mock_engine
ETH_CONNECTOR_DIR = mock_eth_connector
ENGINE_MOCK_DIR = ${MOCK_DIR}/${ENGINE_DIR}
ETH_CONNECTOR_MOCK_DIR = ${MOCK_DIR}/${ETH_CONNECTOR_DIR}
MOCK_CARGO_BUILD = RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

ENGINE_MOCK_FILE = ${ENGINE_MOCK_DIR}/target/wasm32-unknown-unknown/release/mock_engine.wasm
ETH_CONNECTOR_MOCK_FILE = ${ETH_CONNECTOR_MOCK_DIR}/target/wasm32-unknown-unknown/release/mock_eth_connector.wasm

check: check-fmt clippy

clippy: clippy-lib clippy-mock-engine clippy-mock-eth-connector

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

create-bin-dir:
	@mkdir -p bin || true

cp-built-mocks: create-bin-dir
	@cp -rf ${ENGINE_MOCK_FILE} bin/
	@cp -rf ${ETH_CONNECTOR_MOCK_FILE} bin/

test-engine:
	@cargo test --package aurora-workspace-engine -- --test-threads 10 --nocapture

test-eth-connector:
	@cargo test --package aurora-workspace-eth-connector -- --test-threads 10 --nocapture

test-flow: test-engine test-eth-connector

test: build-mock-engine build-mock-eth-connector cp-built-mocks test-flow
