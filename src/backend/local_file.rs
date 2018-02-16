use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use backend::{Backend, BkMeta};

pub struct LocalFile {
    id: String,
    path: String,
}

impl LocalFile {
    pub fn new(id: String, path: String) -> Self {
        LocalFile { 
            id: id,
       path: path,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_meta_file_name(key: &str) -> String {
        format!("{}.info", key)
    }

    fn write_file(&self, key: &str, data: String) -> Result<(), ()> {
        let p = &format!("{}/{}",self.path, &key);
        println!("{}",p);
        if let Ok(mut file) = File::create(p) {
            file.write_all(data.as_bytes());
            Ok(())
        } else {
            Err(())
        }
    }

    fn read_file(&self, key: &str) -> Result<String, ()> {
        if let Ok(mut file) = File::open(&format!("{}/{}", self.path, &key)) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(contents)
        } else {
            Err(())
        }
    }

    fn write_lock(&self, key: &str, should_be_locked: bool) {
        self.write_file(&format!("{}.lock", &key), should_be_locked.to_string());
    }

    fn read_lock(&self, key: &str) -> bool {
        if let Ok(read) = self.read_file(&format!("{}.lock", &key)) {
            FromStr::from_str(&read).unwrap()
        } else {
            // if file doesn't exist then create it and return lock is available
            self.write_lock(&key, false);
            false
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
        if let Ok(_) = self.write_file(&key, data.get_data().to_string()) {
            true
        } else {
            false
        }
    }

    // this is always the lates version for now
    fn get_key(&self, key: String) -> String {
        if let Ok(read) = self.read_file(&key) {
            read
        } else {
            //FIXME: doesnt exist
            "".to_string()
        }
    }

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self, key: String) -> BkMeta {
        println!("==========--- {}", &key);
        let meta_file_name = LocalFile::get_meta_file_name(&key);
        if let Ok(read) = self.read_file(&meta_file_name) {
            println!("==========0");
            println!("==========0{}", read);
            //FIXME from_str caused errors!!!!! investigate later
            use serde_json;
            let d = serde_json::from_str(&read).unwrap();
            // let d =BkMeta::from_str(&read).expect("file malformed");
            println!("{:?}", d);
            d
        } else {
            println!("==========1");
            BkMeta::init()
        }
    }

    fn set_meta(&self, meta: String, key: String) -> bool {
        let meta_file_name = LocalFile::get_meta_file_name(&key);
        if let Ok(_) = self.write_file(&meta_file_name, meta.to_string()) {
            true
        } else {
            false
        }
    }

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self, key: String) -> bool {
        if self.read_lock(&key) {
            false
        } else {
            self.write_lock(&key, true);
            true
        }
    }

    fn release_lock(&self, key: String) -> bool {
        self.write_lock(&key, false);
        true
    }
}
