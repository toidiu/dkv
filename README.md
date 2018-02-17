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

## Running

In this demo version the "Backends" are simply folders at the project/executable root. The name of the folders can be configured in `server.rs` (**s3** and **dropbox**)
**The folders must exist for the server to work properly.**

- Create folders `make createLocalS3` and `make createLocalDropbox`
- run server `make server`
- run clients `make set` and `make get` (currently there is minimal error handling so please set values before getting them. We can only get the latest value at the moment.. hack day)
- test deleting and re-creating one of the backends `make deleteLocalS3` and `make deleteLocalDropbox`
