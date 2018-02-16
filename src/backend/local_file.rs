use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use backend::{Backend, BkMeta};

pub struct LocalFile {
    id: String,
}

impl LocalFile {
    pub fn new(id: String) -> Self {
        LocalFile { id: id }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    fn write_lock(key: &str, should_be_locked: bool) {
        LocalFile::write_file(&key, should_be_locked.to_string());
    }

    fn read_lock(key: &str) -> bool {
        if let Ok(read) = LocalFile::read_file(&key) {
            FromStr::from_str(&read).unwrap()
        } else {
            // if file doesn't exist then create it and return lock is available
            LocalFile::write_lock(&key, false);
            false
        }
    }

    fn write_file(key: &str, data: String) -> Result<(), ()> {
        if let Ok(mut file) = File::create(&key) {
            file.write_all(data.as_bytes());
            Ok(())
        } else {
            Err(())
        }
    }

    fn read_file(key: &str) -> Result<String, ()> {
        if let Ok(mut file) = File::open(&key) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(contents)
        } else {
            Err(())
        }
    }
}

impl Backend for LocalFile {
    //== must be unique
    fn id(&self) -> String {
        self.id.clone()
    }

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, key: String) -> bool {
        if let Ok(_) = LocalFile::write_file(&key, data.get_data().to_string()) {
        true
        } else {
            false
        }
    }

    // this is always the lates version for now
    fn get_key(&self, key: String) -> String {
        if let Ok(read) = LocalFile::read_file(&key) {
        read
        } else {
            //FIXME: doesnt exist
            "".to_string()
        }
    }

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self, key: String) -> BkMeta {
        if let Ok(read) = LocalFile::read_file(&key) {
            BkMeta::from_str(&read).expect("file malformed")
        } else {
            BkMeta::init()
        }
    }

    fn set_meta(&self, meta: String, key: String) -> bool {
        if let Ok(_) = LocalFile::write_file(&key, meta.to_string()) {
        true
        } else {
            false
        }
    }

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self, key: String) -> bool {
        if LocalFile::read_lock(&key) {
            false
        } else {
            LocalFile::write_lock(&key, true);
            true
        }
    }

    fn release_lock(&self, key: String) -> bool {
        LocalFile::write_lock(&key, false);
        true
    }
}
