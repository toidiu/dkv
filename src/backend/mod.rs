use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

pub type BkSend = Backend + Send + Sync;

mod in_mem;

pub trait Backend {
    //== must be unique
    fn id(&self) -> String;

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, obj_name: String) -> bool;
    // this is always the lates version for now
    fn get_key(&self, obj_name: String) -> String;

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self) -> String;
    fn set_meta(&self, meta: String, obj_name: String) -> bool;

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self) -> bool;
    fn release_lock(&self) -> bool;
}
