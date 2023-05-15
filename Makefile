CLIPPY_RULES = -D warnings

MOCK_DIR = res
ENGINE_DIR = mock_engine
ETH_CONNECTOR_DIR = mock_eth_connector
ENGINE_MOCK_DIR = ${MOCK_DIR}/${ENGINE_DIR}/
ETH_CONNECTOR_MOCK_DIR = ${MOCK_DIR}/${ETH_CONNECTOR_DIR}/
MOCK_CARGO_BUILD = RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

ENGINE_MOCK_FILE = ${ENGINE_MOCK_DIR}target/wasm32-unknown-unknown/release/mock_engine.wasm
ETH_CONNECTOR_MOCK_FILE = ${ETH_CONNECTOR_MOCK_DIR}target/wasm32-unknown-unknown/release/mock_eth_connector.wasm

check: check-fmt clippy

clippy: clippy-lib clippy-mock-engine clippy-mock-eth-connector clippy-test

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

cp-builded-mocks: create-bin-dir
	@cp ${ENGINE_MOCK_FILE} bin/
	@cp ${ETH_CONNECTOR_MOCK_FILE} bin/

test-flow:
	@cargo test --all -- --nocapture

test: build-mock-engine build-mock-eth-connector cp-builded-mocks test-flow
