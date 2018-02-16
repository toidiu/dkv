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

use std::collections::HashMap;
use dkv::distributed;
use dkv::backend;
use dkv::init_log;
use std::io::Read;
use std::sync::{Arc, Mutex};
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
    backends: Arc<Mutex<HashMap<String, Box<backend::BkSend>>>>,

    total_backends: u32,
}

impl Dkv for MyDkvService {
    fn add_key(&self, ctx: RpcContext, req: AddKeyRequest, sink: UnarySink<AddKeyReply>) {
        info!("set key");
        let msg = format!("success!");
        let mut resp = AddKeyReply::new();
        let mut status = Status::new();

        let add_status =
            distributed::distributed_add(req.clone(), self.total_backends, self.backends.clone());
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
        info!("get key");
        let msg = format!("success!");
        let mut resp = GetKeyReply::new();

        let mut status = Status::new();

        match distributed::distributed_get(
            req.get_key().to_string(),
            self.total_backends,
            self.backends.clone(),
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

    let total_backends = 1;
    let bk_local_file1 =
        backend::local_file::LocalFile::new("local-file".to_string(), "store1".to_string());
    let bk_local_file2 =
        backend::local_file::LocalFile::new("local-file2".to_string(), "store2".to_string());

    let mut map: HashMap<String, Box<backend::BkSend>> = HashMap::new();
    map.insert(bk_local_file1.get_id(), Box::new(bk_local_file1));
    map.insert(bk_local_file2.get_id(), Box::new(bk_local_file2));

    let my_service = MyDkvService {
        backends: Arc::new(Mutex::new(map)),
        total_backends: total_backends,
    };

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
