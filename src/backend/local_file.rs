use std::sync::Mutex;
use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

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

    fn write_lock(key: &str, should_locked: bool) {
        let mut file = File::create(&key).unwrap();
        file.write_all(should_locked.to_string().as_bytes());
    }

    fn read_lock(key: &str) -> bool {
        let mut file = File::open(&key).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        FromStr::from_str(&contents).unwrap()
    }
}

impl ::backend::Backend for LocalFile {
    //== must be unique
    fn id(&self) -> String {
        self.id.clone()
    }

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, key: String) -> bool {
        // MEMORY
        //     .lock()
        //     .unwrap()
        //     .insert(key, data.get_data().to_string());
        let mut file = File::create(&key).unwrap();
        file.write_all(data.get_data().as_bytes());
        true
    }

    // this is always the lates version for now
    fn get_key(&self, key: String) -> String {
        // MEMORY
        //     .lock()
        //     .unwrap()
        //     .get(&key)
        //     .unwrap_or(&"".to_string())
        //     .to_owned()
        let mut file = File::open(&key).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self, key: String) -> String {
        // META.lock()
        //     .unwrap()
        //     .get(&key)
        //     .unwrap_or(&"".to_string())
        //     .to_owned()
        let mut file = File::open(&key).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn set_meta(&self, meta: String, key: String) -> bool {
        // META.lock().unwrap().insert(key, meta);
        let mut file = File::create(&key).unwrap();
        file.write_all(meta.as_bytes());
        true
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
