#![feature(proc_macro)]

extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use hyper::Client;
use std::collections::HashMap;

// #[derive(Deserialize)]
// pub struct Vendor {
//     name: String
// }

#[derive(Deserialize)]
pub struct Welcome {
    couchdb: String,
    version: String,
    //vendor: Vendor
    vendor: HashMap<String, String>
}
//{"couchdb":"Welcome","version":"ac3dae3","vendor":{"name":"The Apache Software Foundation"}}


fn main () {
    let mut buf = String::new();
    let client = Client::new();
    let mut res = client.get("http://127.0.0.1:5984").send().unwrap();
    res.read_to_string(&mut buf).unwrap();
    match serde_json::from_str::<Welcome>(&buf) {
        Ok(welcome) => println!("{} version {} with vendor {}", welcome.couchdb, welcome.version, welcome.vendor.get("name").unwrap()),
        _ => println!("errrr")
    };
    println!("res {}", buf);
}
