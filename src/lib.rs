#![allow(unused)]

extern crate grpcio_proto;
#[macro_use]
extern crate lazy_static;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

use std::fs::File;
use self::slog::{Drain, Logger, OwnedKV};
use self::slog_scope::GlobalLoggerGuard;
use self::slog_term::{Decorator, FullFormat, PlainSyncDecorator, TermDecorator};

pub mod backend;
pub mod distributed;

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
