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

use grpcio_proto::dkv::dkv::ResGetKeyValue;

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


pub trait Backend  {

    // 'key.version' file that stores data for that particular version
    fn add_key(&self) -> bool;
    // fn get_key(&self) -> String;
    //
    // 'key.info' file that stores all information about kv
    // fn get_meta(&self) -> String;
    // fn set_meta(&self) -> String;
    //
    // 'key.lock' that indicates atomic access
    // fn get_lock(&self);
    // fn set_lock(&self);

}

pub struct S3 {

}

impl Backend for S3 {
    fn add_key(&self) -> bool {
        true
    }

}


pub fn distributed_add(backends: Arc<Vec<Box<Backend + Send + Sync>>>) -> bool {
    true
}

pub fn distributed_get(backends: Arc<Vec<Box<Backend + Send + Sync>>>) -> Result<ResGetKeyValue, RepeatedField<String>> {
    let mut val = ResGetKeyValue::new();
    val.set_data("this is fake data".to_string());
    val.set_version("this is fake version".to_string());
    val.set_key("this is fake key".to_string());
    Ok(val)
}


