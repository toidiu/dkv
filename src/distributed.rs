use std::sync::Arc;
use std::collections::HashMap;
use protobuf::repeated::RepeatedField;
use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

use backend::BkSend;

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
    // let mut bk_id_latest: String = "".to_string();
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
                // bk_id_latest = bk_id.clone();
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
        //FIXME use serde to format the data and store it
        let updated_meta = format!("{}:{}", meta, max_version);

        // save meta
        let meta_file_name = format!("{}.{}", data.get_key(), max_version);
        bk.set_meta(updated_meta, meta_file_name);
    }

    //== release lock
    for bk in &bk_locks {
        bk.release_lock();
    }

    Ok(bk_locks)
}

pub fn distributed_get(
    key: String,
    total_backends: usize,
    arc_backends: Arc<Vec<Box<BkSend>>>,
) -> Result<ResGetKeyValue, RepeatedField<String>> {
    //== attempt to acquire Arc
    let mut bk_list = Vec::new();
    // See if we can acquire Arc else fail... this should go away once we
    // replace Arc with custom wrapper
    match Arc::try_unwrap(arc_backends) {
        Ok(b_vec) => bk_list = b_vec,

        Err(e) => {
            return Err(RepeatedField::from_vec(vec![
                "unable to acquire Arc".to_string(),
            ]))
        }
    }

    // check if it exists
    //FIXME assume it does

    //== attempt to acquire locks
    let mut bk_locks: HashMap<String, Box<BkSend>> = HashMap::new();
    for bk in bk_list {
        // acquire lock
        if bk.acquire_lock() {
            bk_locks.insert(bk.id(), bk);
        };
    }

    //== if we dont have enough locks then release all locks and abort
    if bk_locks.len() <= total_backends / 2 {
        for bk in bk_locks.values() {
            bk.release_lock();
        }

        return Err(RepeatedField::from_vec(vec![
            "unable to acquire enough locks".to_string(),
        ]));
    }

    ////== figure out which backend had the latest data.
    let mut max_version = 0;
    let mut bk_id_latest: String = "".to_string();
    let mut bkid_ver_map: HashMap<String, usize> = HashMap::new();
    let mut bkid_meta_map: HashMap<String, String> = HashMap::new();
    {
        //== we have enough locks so lets try to add value
        //FIXME make this concurrent
        for bk in bk_locks.values() {
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
                bk_id_latest = bk_id.clone();
            }
        }
    }

    //== get value from Backend bk_id_latest
    let bk_latest = bk_locks
        .get(&bk_id_latest)
        .expect("should exist")
        .to_owned();
    let file_name = format!("{}.{}", key, max_version);
    let data = bk_latest.get_key(file_name);

    let mut val = ResGetKeyValue::new();
    val.set_data(data);
    val.set_version(max_version.to_string());
    val.set_key(key);

    ////FIXME optional (update backends with latest)

    //== release lock
    for bk in bk_locks.values() {
        bk.release_lock();
    }

    Ok(val)
}

pub mod backend {

    extern crate grpcio_proto;

    use std::sync::Arc;
    use protobuf::repeated::RepeatedField;
    use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

    pub type BkSend = Backend + Send + Sync;

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
}
