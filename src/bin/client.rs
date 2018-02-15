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
use grpcio_proto::dkv::dkv_grpc::{self, DkvClient};
use grpcio_proto::dkv::dkv::{
  Status,
  AddKeyRequest,
  GetKeyRequest,
  GetKeyReply,
  AddKeyReply
};


fn main() {
    let _guard = init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = DkvClient::new(ch);

    let mut req = AddKeyRequest::new();
//     req.set_name("World".to_owned());
    let reply = client.add_key(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_status().get_success());
}
