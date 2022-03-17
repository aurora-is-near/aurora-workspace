# Eth Connector Contract mock

It covers Eth Connector contract public methods. Through NEAR
blockchain it's possible to get access only to public methods and
view functions.


### How to generate the WASM file

- Install Rust and add the wasm32-unknown-unknown target
```bash
rustup target add wasm32-unknown-unknown 
``` 


- Build the wasm file
```bash
make build-eth-connector-mock
```

You should find the `mock_eth_connector.wasm` file in `../bin/mock_eth_connector.wasm`.
