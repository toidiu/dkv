#![allow(unused)]

extern crate slog;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;
extern crate grpcio_proto;
extern crate protobuf;

use std::fs::File;
use std::sync::Arc;

use protobuf::repeated::RepeatedField;

use self::slog::{Drain, Logger, OwnedKV};
use self::slog_scope::GlobalLoggerGuard;
use self::slog_term::{Decorator, FullFormat, PlainSyncDecorator, TermDecorator};

use grpcio_proto::dkv::dkv::{
    ResGetKeyValue,
    AddKeyRequest,
};

pub fn init_log(log_file: Option<String>) -> GlobalLoggerGuard {
    fn setup<D: Decorator + Send + 'static>(decorator: D) -> GlobalLoggerGuard {
        let drain = FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = Logger::root(drain, OwnedKV(()));
        let guard = slog_scope::set_global_logger(logger);
        slog_stdlog::init().unwrap();
        guard
    }

    match log_file {
        Some(path) => {
            let file = File::create(path).unwrap();
            setup(PlainSyncDecorator::new(file))
        }
        None => setup(TermDecorator::new().build()),
    }
}

pub type BkSend = Backend + Send + Sync;

pub trait Backend  {

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest) -> bool;
    // this is always the lates version for now
    fn get_key(&self) -> String;

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self) -> String;
    fn set_meta(&self) -> bool;

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self) -> bool;
    fn release_lock(&self) -> bool;
}

// pub struct S3 {
// }

// impl Backend for S3 {
// }


pub fn distributed_add(
    data: AddKeyRequest,
    total_backends: usize,
    arc_backends: Arc<Vec<Box<BkSend>>>
) -> bool {

    //== attempt to acquire Arc
    let mut bk_list = Vec::new();
    // See if we can acquire Arc else fail... this should go away once we 
    // replace Arc with custom wrapper
    match Arc::try_unwrap(arc_backends) {
        Ok(b_vec) => bk_list = b_vec,

        Err(e) => return false
    }

    //== attempt to acquire locks
    let mut bk_locks: Vec<Box<BkSend>> = Vec::new();
    for bk in bk_list {
        // acquire lock
        if bk.acquire_lock() {
            bk_locks.push(bk);
        };
    }

    //== if we dont have enough locks then release all locks and abort
    if bk_locks.len() <= total_backends/2 {
        for bk in bk_locks {
            bk.release_lock();
        }
        return false
    }


    //== we have enough locks so lets try to add value
    let mut meta_list: Vec<(Box<BkSend>, usize)> = Vec::new();
    for bk in bk_locks {
        // get meta.version
        let meta = bk.get_meta();
        let version = 1; // fake for now
        meta_list.push((bk, version));
    }

    // for bk in bk_locks {
    //     // set data /w key.version++
    //     bk.add_key(data);

    //     // set meta.append(new version info)
    //     let meta = meta;//.version++;

    //     // release lock
    // }

    true
}

pub fn distributed_get(
    total_backends: usize,
    arc_backends: Arc<Vec<Box<BkSend>>>
    ) -> Result<ResGetKeyValue, RepeatedField<String>> {
    let mut val = ResGetKeyValue::new();
    val.set_data("this is fake data".to_string());
    val.set_version("this is fake version".to_string());
    val.set_key("this is fake key".to_string());
    Ok(val)
}


