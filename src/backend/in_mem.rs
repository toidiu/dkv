//use std::sync::Mutex;
//use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};
//use std::collections::HashMap;
//use ::backend::{Backend, BkMeta};

//lazy_static! {
//    static ref META: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
//    static ref MEMORY: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
//    static ref LOCK: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
//}

//pub struct InMem {
//    id: String,
//}

//impl InMem {
//    pub fn new(id: String) -> Self {
//        InMem { id: id }
//    }

//    pub fn get_id(&self) -> String {
//        self.id.clone()
//    }
//}

//impl Backend for InMem {
//    //== must be unique
//    fn id(&self) -> String {
//        self.id.clone()
//    }

//    //== 'key.version' file that stores data for that particular version
//    // this is always the next version
//    fn add_key(&self, data: &AddKeyRequest, key: String) -> bool {
//        MEMORY
//            .lock()
//            .unwrap()
//            .insert(key, data.get_data().to_string());
//        true
//    }

//    // this is always the lates version for now
//    fn get_key(&self, key: String) -> String {
//        MEMORY
//            .lock()
//            .unwrap()
//            .get(&key)
//            .unwrap_or(&"".to_string())
//            .to_owned()
//    }

//    //== 'key.meta' file that stores all information about kv
//    fn get_meta(&self, key: String) -> BkMeta {
//        META.lock()
//            .unwrap()
//            .get(&key)
//            .unwrap_or(&"".to_string())
//            .to_owned()
//    }
//    fn set_meta(&self, meta: String, key: String) -> bool {
//        META.lock().unwrap().insert(key, meta);
//        true
//    }

//    //== 'key.lock' that indicates atomic access
//    // this needs to be an atomic operation
//    fn acquire_lock(&self, key: String) -> bool {
//        if *LOCK.lock().unwrap().get(&key).unwrap_or(&false) {
//            false
//        } else {
//            LOCK.lock().unwrap().insert(key, true);
//            true
//        }
//    }
//    fn release_lock(&self, key: String) -> bool {
//        LOCK.lock().unwrap().insert(key, false);
//        true
//    }
//}
