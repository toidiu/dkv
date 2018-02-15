# grcp-rs-example
A grpc example of using the grpcio crate: https://crates.io/crates/grpcio

## Motivation
The grcpio crate repo (https://github.com/pingcap/grpc-rs) has an examples of how to use the crate but its difficult to image how you might use the crate as a standalone project. This repo attempts to bridge that need. 


## Project Structure / Workflow
The main service project `grpcio-example` lives in *src* while the protobuf dependency module `grpcio-proto` lives in *proto*. 

An overview of the workflow is as follows:
- run `proto/build.sh` to generate your proto desc files
- `cargo build -p grpcio-proto` to build the proto module
- edit project code in `src/bin`
- `cargo build` to generate client and server executables




---
**Disclaimer** I did not write the logic for the server, client, or proto generation. I simply reused part of the existing code at the crates repo and organized it into a standalone project.
