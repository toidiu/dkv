#![allow(unused)]
#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use] extern crate log;
extern crate num_cpus;
extern crate dkv;

use dkv::init_log;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use grpcio_proto::dkv::dkv_grpc::{self, Dkv};
use grpcio_proto::dkv::dkv::{
  Status,
  AddKeyRequest,
  GetKeyRequest,
  GetKeyReply,
  AddKeyReply
};


#[derive(Clone)]
struct MyDkvService;

impl Dkv for MyDkvService {
    fn add_key(&self, ctx: RpcContext, val: AddKeyRequest, sink: UnarySink<AddKeyReply>) {
        let msg = format!("success!");
        let mut resp = AddKeyReply::new();
        let mut status = Status::new();
        status.set_success(true);

        resp.set_status(status);
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", val, e));
        ctx.spawn(f)
    }

    fn get_key(&self, ctx: RpcContext, val: GetKeyRequest, sink: UnarySink<GetKeyReply>) {

    }

    // fn say_hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
    //     let msg = format!("Hello {}!", req.get_name());
    //     let mut resp = HelloReply::new();
    //     resp.set_message(msg);
    //     let f = sink.success(resp)
    //         .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
    //     ctx.spawn(f)
    // }
}

fn main() {
    let _guard = init_log(None);
    let env = Arc::new(Environment::new(num_cpus::get()));
    let service = dkv_grpc::create_dkv(MyDkvService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
