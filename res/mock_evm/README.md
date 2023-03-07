# Generate the WASM file

- Install nightly toolchain 
```bash
rustup toolchain install nightly 
```
- Only for Apple M1/M2 MacOS
```bash
rustup default stable-aarch64-apple-darwin
```
- Add the wasm32-unknown-unknown target for the nightly toolchain 
```bash
rustup target add --toolchain nightly wasm32-unknown-unknown 
```

- Build the wasm file
```bash
./build.sh 
```

You should find the `main.wasm` file in `../bin/main.wasm`.