# aurora-workspace

Aurora Workspace library for the Aurora Engine contract based on NEAR Protocol.

Easy to use as workspace utility to interact and tests
Aurora Engine compatible NEAR contracts.


## Minimum Supported Rust Version (MSRV)

This library is only compatible with Rust version 1.64.0 and above.

## Useful commands for development

- `make check` - run clippy for all code and mock 
  projects. Check formatting.
- `make test` - build mock projects and run tests
- `make test-flow` - run tests only (mock project should be build)


## Tests

Before running tests, build/generate the WASM file as described [here](res/mock_engine/README.md).
```
make test
```

## LICENCE
[CCO-1.0](LICENSE)
