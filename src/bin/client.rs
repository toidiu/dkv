#![allow(unused)]

extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate dkv;

use std::sync::Arc;

use dkv::init_log;
use grpcio::{ChannelBuilder, EnvBuilder};

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
    req.set_key("key1".to_string());
    req.set_data("data1".to_string());
    let reply = client.add_key(&req).expect("rpc");
    info!("Status of add was: {}", reply.get_status().get_success());

    let mut req = GetKeyRequest::new();
    let reply = client.get_key(&req).expect("rpc");
    info!("Status of get was: {} and value was: {:?}", reply.get_status().get_success(), reply.get_val());

}
