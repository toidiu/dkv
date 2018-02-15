// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate futures;
extern crate grpcio;
extern crate protobuf;

pub mod testing {
    include!(concat!(env!("OUT_DIR"), "/testing/mod.rs"));
}

pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/greeter/mod.rs"));
}

pub mod dkv {
    include!(concat!(env!("OUT_DIR"), "/dkv/mod.rs"));
}

pub mod health {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/health/mod.rs"));
    }
}

pub mod util;
