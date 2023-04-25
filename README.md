<p>&nbsp;</p>
<p align="center">
<img src="https://raw.githubusercontent.com/aurora-is-near/aurora-workspace/main/res/aurora-workspace-logo.svg" width=500>
</p>

# aurora-workspace

Aurora Workspace is a library for the **Aurora Engine** and Aurora 
**Eth Connector** and EVM contracts based on **NEAR Protocol**.

Easy to use as workspace utility to interact and tests
Aurora Engine compatible NEAR contracts.

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

## Minimum Supported Rust Version (MSRV)

This library is only compatible with Rust version 1.64.0 and above.

## LICENCE
[CCO-1.0](LICENSE)
