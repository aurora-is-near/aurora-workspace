# Engine Contract mock

It covers Aurora Engine contract public methods. Through NEAR 
blockchain it's possible to get access only to public methods and
view functions.


### How to generate the WASM file

- Install Rust and add the wasm32-unknown-unknown target
```bash
rustup target add wasm32-unknown-unknown 
```

- Build the wasm file
```bash
make build-mock-engine
```

You should find the `mock_engine.wasm` file in `../bin/mock_engine.wasm`.
