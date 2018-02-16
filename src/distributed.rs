use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use protobuf::repeated::RepeatedField;
use grpcio_proto::dkv::dkv::{AddKeyRequest, ResGetKeyValue};

use backend::{BkMeta, BkSend};

pub fn distributed_add(
    data: AddKeyRequest,
    total_backends: u32,
    arc_backends: Arc<Mutex<HashMap<String, Box<BkSend>>>>,
    // arc_backends: Arc<Vec<Box<BkSend>>>,
) -> Result<Vec<String>, RepeatedField<String>> {
    //== attempt to acquire Mutex and block until
    let bk_map = &*arc_backends.lock().unwrap();

    //== attempt to acquire locks
    let mut bk_id_locks: Vec<String> = Vec::new();
    for (bk_id, bk) in bk_map {
        // acquire lock
        if bk.acquire_lock(data.get_key().to_string()) {
            bk_id_locks.push(bk_id.clone());
        };
    }

    //== if we dont have enough locks then release all locks and abort
    if bk_id_locks.len() as u32 <= (total_backends / 2) {
        for bk_id in &bk_id_locks {
            bk_map
                .get(bk_id)
                .unwrap()
                .release_lock(data.get_key().to_string());
        }

        return Err(RepeatedField::from_vec(vec![
            "unable to acquire enough locks".to_string(),
        ]));
    }

    //== figure out which backend had the latest data.
    let mut max_version = 0;
    let mut bk_id_latest: String = "".to_string();
    let mut bkid_meta_map: HashMap<String, BkMeta> = HashMap::new();
    {
        let mut bkid_ver_map: HashMap<String, u32> = HashMap::new();
        //== we have enough locks so lets try to add value
        //FIXME make this concurrent
        for bk_id in &bk_id_locks {
            // get meta.version
            let meta = bk_map
                .get(bk_id)
                .unwrap()
                .get_meta(data.get_key().to_string());
            println!("{:?}", meta);
            let version = meta.latest_version;

            println!("======ver {}", version);
            if max_version < version {
                max_version = version
            }

            bkid_meta_map.insert(bk_map.get(bk_id).unwrap().id(), meta);
            bkid_ver_map.insert(bk_map.get(bk_id).unwrap().id(), version);
        }

        ////== compare versions
        //for (bk_id, ver) in &bkid_ver_map {
        //    if *ver > max_version {
        //        max_version = *ver;
        //        bk_id_latest = bk_id.clone();
        //    }
        //}
    }

    println!("======max {}", max_version);
    let max_version = max_version + 1;
    //== Add the data with the latest version++ to
    // all the backends
    for bk_id in &bk_id_locks {
        // save data
        let file_name = format!("{}.{}", data.get_key(), max_version);
        bk_map.get(bk_id).unwrap().add_key(&data, file_name);

        // get meta and append new version info
        let meta = bkid_meta_map.get_mut(bk_id).expect("should exist");
        println!("======max {}", max_version);
        meta.add_version(max_version);
        println!("======meta {:?}", meta);

        // save meta
        bk_map
            .get(bk_id)
            .unwrap()
            .set_meta(meta.to_string(), data.get_key().to_string());
    }

    //== release lock
    for bk_id in &bk_id_locks {
        bk_map
            .get(bk_id)
            .unwrap()
            .release_lock(data.get_key().to_string());
    }

    Ok(bk_id_locks)
}

/// The core tenet of the Borrowing system is: Aliasing XOR Mutability.
/// Aliasing: variables and pointers alias if they refer to overlapping regions of memory.
/// Mutability: chaning the data
///
/// So we can't mutate data from two places.
/// Arc prevents mutability; we can't get a mutable reference to data inside of an Arc.
/// Mutex prevents aliasing; we can only mutate data from one place at a time.
pub fn distributed_get(
    key: String,
    total_backends: u32,
    arc_backends: Arc<Mutex<HashMap<String, Box<BkSend>>>>,
) -> Result<ResGetKeyValue, RepeatedField<String>> {
    //== attempt to acquire Mutex and block until
    let bk_map = &*arc_backends.lock().unwrap();

    // check if it exists
    //FIXME assume it does

    //== attempt to acquire locks
    let mut bk_id_locks: Vec<String> = Vec::new();
    for (bk_id, bk) in bk_map {
        // acquire lock
        if bk.acquire_lock(key.clone()) {
            bk_id_locks.push(bk_id.clone());
        };
    }

    //== if we dont have enough locks then release all locks and abort
    if bk_id_locks.len() as u32 <= total_backends / 2 {
        for bk_id in &bk_id_locks {
            bk_map.get(bk_id).unwrap().release_lock(key.clone());
        }

        return Err(RepeatedField::from_vec(vec![
            "unable to acquire enough locks".to_string(),
        ]));
    }

    //== figure out which backend had the latest data.
    let mut max_version = 0;
    let mut bk_id_latest: String = "".to_string();
    {
        let mut bkid_meta_map: HashMap<String, String> = HashMap::new();
        let mut bkid_ver_map: HashMap<String, u32> = HashMap::new();
        //== we have enough locks so lets try to add value
        //FIXME make this concurrent
        for bk_id in &bk_id_locks {
            // get meta.version
            let meta = bk_map.get(bk_id).unwrap().get_meta(key.to_string());
            let version = meta.latest_version;
            bkid_meta_map.insert(bk_map.get(bk_id).unwrap().id(), meta.to_string());
            bkid_ver_map.insert(bk_map.get(bk_id).unwrap().id(), version);
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
    let bk_latest = bk_map.get(&bk_id_latest).expect("should exist").to_owned();
    let file_name = format!("{}.{}", key.clone(), max_version);
    let data = bk_latest.get_key(file_name);

    let mut val = ResGetKeyValue::new();
    val.set_data(data);
    val.set_version(max_version.to_string());
    val.set_key(key.clone());

    //FIXME optional (update backends with latest)

    //== release lock
    for bk_id in &bk_id_locks {
        bk_map.get(bk_id).unwrap().release_lock(key.clone());
    }

    Ok(val)
}
