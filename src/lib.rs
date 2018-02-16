#![allow(unused)]

extern crate grpcio_proto;
extern crate protobuf;
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;

use protobuf::repeated::RepeatedField;

use self::slog::{Drain, Logger, OwnedKV};
use self::slog_scope::GlobalLoggerGuard;
use self::slog_term::{Decorator, FullFormat, PlainSyncDecorator, TermDecorator};

use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

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

pub trait Backend {
    //== must be unique
    fn id(&self) -> String;

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, obj_name: String) -> bool;
    // this is always the lates version for now
    fn get_key(&self) -> String;

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self) -> String;
    fn set_meta(&self, meta: String, obj_name: String) -> bool;

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
    arc_backends: Arc<Vec<Box<BkSend>>>,
) -> Result<Vec<Box<BkSend>>, String> {
    //== attempt to acquire Arc
    let mut bk_list = Vec::new();
    // See if we can acquire Arc else fail... this should go away once we
    // replace Arc with custom wrapper
    match Arc::try_unwrap(arc_backends) {
        Ok(b_vec) => bk_list = b_vec,

        Err(e) => return Err("unable to acquire Arc".to_string()),
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
    if bk_locks.len() <= total_backends / 2 {
        for bk in bk_locks {
            bk.release_lock();
        }
        return Err("unable to acquire enough locks".to_string());
    }

    //== figure out which backend had the latest data.
    let mut max_version = 0;
    let mut latest_backend_id: String = "".to_string();
    let mut bkid_ver_map: HashMap<String, usize> = HashMap::new();
    let mut bkid_meta_map: HashMap<String, String> = HashMap::new();
    {
        //== we have enough locks so lets try to add value
        //FIXME make this concurrent
        for bk in &bk_locks {
            // get meta.version
            let meta = bk.get_meta();
            let version = 1; //FIXME fake for now
            bkid_meta_map.insert(bk.id(), meta);
            bkid_ver_map.insert(bk.id(), version);
        }

        //== compare versions
        for (bk_id, ver) in &bkid_ver_map {
            if *ver > max_version {
                max_version = *ver;
                latest_backend_id = bk_id.clone();
            }
        }
    }

    //== Add the data with the latest version++ to
    // all the backends
    for bk in &bk_locks {
        // save data
        let file_name = format!("{}.{}", data.get_key(), max_version);
        bk.add_key(&data, file_name);

        // get meta and append new version info
        let meta = bkid_meta_map
            .get(&bk.id())
            .expect("should exist")
            .to_owned();
        //FIXME
        let meta = format!("{} {}", meta, latest_backend_id);

        // save meta
        let meta_file_name = format!("{}.{}", data.get_key(), max_version);
        bk.set_meta(meta, meta_file_name);

        // release lock
        bk.release_lock();
    }

    Ok(bk_locks)
}

pub fn distributed_get(
    total_backends: usize,
    arc_backends: Arc<Vec<Box<BkSend>>>,
) -> Result<ResGetKeyValue, RepeatedField<String>> {
    let mut val = ResGetKeyValue::new();
    val.set_data("this is fake data".to_string());
    val.set_version("this is fake version".to_string());
    val.set_key("this is fake key".to_string());
    Ok(val)
}
