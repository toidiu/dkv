#![allow(unused)]
#![allow(unknown_lints)]
#![allow(unreadable_literal)]
#![feature(underscore_lifetimes)]

extern crate dkv;
extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate num_cpus;

use dkv::init_log;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use grpcio_proto::dkv::dkv_grpc::{self, Dkv};
use grpcio_proto::dkv::dkv::{AddKeyReply, AddKeyRequest, GetKeyReply, GetKeyRequest,
                             ResGetKeyValue, Status};

#[derive(Clone)]
struct MyDkvService {
    /// FIXME: using Arc for simplification but we will be managing locking
    /// ourselves so there can be a wrapper type around Vec that implements
    /// Send + Sync and doesnt need to do an atomic lock.
    ///
    /// In the current implementation we only lose the multi threaded
    /// benefit of a single instance of the server but the distributed
    /// logic of multiple servers will still work.
    backends: Arc<Vec<Box<dkv::BkSend>>>,

    total_backends: usize,
}

impl Dkv for MyDkvService {
    fn add_key(&self, ctx: RpcContext, req: AddKeyRequest, sink: UnarySink<AddKeyReply>) {
        let msg = format!("success!");
        let mut resp = AddKeyReply::new();
        let mut status = Status::new();

        let add_status =
            dkv::distributed_add(req.clone(), self.total_backends, Arc::clone(&self.backends));
        if let Ok(_) = add_status {
            status.set_success(true);
        } else {
            status.set_success(false);
        }

        resp.set_status(status);
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn get_key(&self, ctx: RpcContext, req: GetKeyRequest, sink: UnarySink<GetKeyReply>) {
        let msg = format!("success!");
        let mut resp = GetKeyReply::new();

        let mut status = Status::new();

        match dkv::distributed_get(
            req.get_key().to_string(),
            self.total_backends,
            Arc::clone(&self.backends),
        ) {
            Ok(val) => {
                status.set_success(true);
                resp.set_val(val);
            }

            Err(e_msg) => {
                status.set_success(false);
                status.set_msg(e_msg);
            }
        }

        resp.set_status(status);
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let _guard = init_log(None);
    let env = Arc::new(Environment::new(num_cpus::get()));

    let total_backends = 2;
    let my_service = MyDkvService {
        backends: Arc::new(Vec::new()),
        total_backends: total_backends,
    };
    // FIXME

    let service = dkv_grpc::create_dkv(my_service);
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
