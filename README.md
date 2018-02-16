# dkv
A distributed key value store. Written in Rust. Utilizes gRPC framework.

Note: this was written during a 24hr hack-day event and is NOT production ready.

## Motivation
Abstract the problem of KV storage. Add redundency to storage. Add versioning to storage. Learn Rust better.

The vision was to depoy dkv as a side-car; which would be possible due to Rust's low memoory footprint. This would allow an abstraction of "storage" which would be upgradable on a servive-to-service basis.

## Project Structure / Workflow
The main service project `dkv` lives in *src* while the protobuf dependency module `grpcio-proto` lives in *proto*.

An overview of the workflow is as follows:
- run `proto/build.sh` to generate your proto desc files
- `cargo build -p grpcio-proto` to build the proto module
- edit project code in `src/bin` and `src/lib`
- `cargo run --bin server` to run server
- `cargo run --bin client_set` to add key to backends
- `cargo run --bin client_get` to get key from backends


In this demo version the "Backends" are simply folders at the project/executable root. The name of the folders can be configured in `server.rs`
**The folders must exist for the server to work properly.**
