# Alloy to Reth types conversion - Example
To test reth ExEx, it can be useful to fetch blocks and transactions using an RPC and forward those to Reth. This repository illustrates the current issues faced when trying to convert Alloy types to Reth types.

## Known Issues
- Optimism is broken due to: https://github.com/alloy-rs/op-alloy/pull/427
- Traits too strict
- Add feature:
  - Get Header
  - Get Transaction

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).