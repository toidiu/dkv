
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate dkv;

use std::sync::Arc;

use dkv::init_log;
use grpcio::{ChannelBuilder, EnvBuilder};

use grpcio_proto::greeter::helloworld::HelloRequest;
use grpcio_proto::greeter::helloworld_grpc::GreeterClient;


fn main() {
    let _guard = init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("World".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}
