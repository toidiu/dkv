use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

pub struct InMem {}

impl ::backend::Backend for InMem {
    //== must be unique
    fn id(&self) -> String {
        "in-mem".to_string()
    }

    //== 'key.version' file that stores data for that particular version
    // this is always the next version
    fn add_key(&self, data: &AddKeyRequest, obj_name: String) -> bool {
        true
    }

    // this is always the lates version for now
    fn get_key(&self, obj_name: String) -> String {
        "key".to_string()
    }

    //== 'key.meta' file that stores all information about kv
    fn get_meta(&self) -> String {
        "meta".to_string()
    }
    fn set_meta(&self, meta: String, obj_name: String) -> bool {
        true
    }

    //== 'key.lock' that indicates atomic access
    // this needs to be an atomic operation
    fn acquire_lock(&self) -> bool {
        true
    }
    fn release_lock(&self) -> bool {
        true
    }
}
