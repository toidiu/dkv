#![allow(unused)]

extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate dkv;

use std::sync::Arc;

use dkv::init_log;
use grpcio::{ChannelBuilder, EnvBuilder};

// use grpcio_proto::dkv::dkv::HelloRequest;
// use grpcio_proto::dkv::dkv_grpc::GreeterClient;


fn main() {
//     let _guard = init_log(None);
//     let env = Arc::new(EnvBuilder::new().build());
//     let ch = ChannelBuilder::new(env).connect("localhost:50051");
//     let client = GreeterClient::new(ch);

//     let mut req = HelloRequest::new();
//     req.set_name("World".to_owned());
//     let reply = client.say_hello(&req).expect("rpc");
//     info!("Greeter received: {}", reply.get_message());
}
