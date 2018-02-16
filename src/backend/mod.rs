use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};
use std::string::ToString;
use std::str::FromStr;
use serde_json;

pub type BkSend = Backend + Send + Sync;

// pub trait BkId{}

// impl BkId for String{}

pub mod in_mem;
pub mod local_file;

pub trait Backend {
    //== must be unique
    fn id(&self) -> String;

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, key: String) -> bool;
    // this is always the lates version for now
    fn get_key(&self, key: String) -> String;

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self, key: String) -> BkMeta;
    fn set_meta(&self, meta: String, key: String) -> bool;

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self, key: String) -> bool;
    fn release_lock(&self, key: String) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BkMeta {
    pub latest_version: u32,
    versions: Vec<u32>,
}

impl BkMeta {
    pub fn init() -> Self {
        BkMeta {
            latest_version: 1,
            versions: Vec::new(),
        }
    }

    pub fn new(ver: u32) -> Self {
        BkMeta {
            latest_version: ver,
            versions: Vec::new(),
        }
    }

    pub fn add_version(&mut self, ver: u32) {
        self.versions.push(ver);
        if self.latest_version < ver {
            self.latest_version = ver;
        }
    }
}

impl ToString for BkMeta {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for BkMeta {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).unwrap()
    }
}
