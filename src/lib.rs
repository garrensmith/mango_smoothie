#![doc(html_root_url = "http://garrensmith.com/mango_smoothie/")]
//! Mango Smoothie

//! Mango Smoothie is a [CouchDB Mango](http://docs.couchdb.org/en/latest/api/database/find.html) /
//! [Cloudant Query](https://docs.cloudant.com/cloudant_query.html) client library.
//!
//! # Create Indexes
//!
//! To create an index first specify the url to the CouchDB/Cloudant instance, then
//! specify the fields to be indexed.
//!
//! ```ignore
//! extern crate mango_smoothie;
//! use mango_smoothie::{database};
//!
//! let resp = database("http://tester:testerpass@127.0.0.1:5984/animaldb").unwrap()
//!           .create_index(&["class", "name"]);
//! ```
//!
//! # View Indexes
//!
//! To list all the available indexes do the following:
//!
//! ``` ignore
//!   let indexes = database("http://tester:testerpass@127.0.0.1:5984/animaldb").unwrap()
//!               .list_indexes().unwrap();
//!
//!   assert!(indexes.total_rows > 0);
//!   assert_eq!(indexes.indexes[0].name, "_all_docs".to_string());
//!   assert!(indexes.indexes[0].def.fields[0].contains_key(&"_id".to_string()));
//! ```
//!
//! # Query Indexes
//!
//! Mango Smoothie uses the [serde_json](https://docs.serde.rs/serde_json/)
//! macro to help with querying indexes.
//!
//! ``` ignore
//! extern crate mango_smoothie;
//! use mango_smoothie::{database};
//! #[macro_use]
//! extern crate serde_json;
//!
//! let query = json!({
//!    "selector": {
//!       "_id": {
//!         "$gt": "1"
//!       }
//!     },
//!     "fields": ["_id", "name"],
//!     "skip": 3,
//!     "sort": [{"_id": "asc"}]
//! });
//!
//! let query_resp = db.query_index(query).unwrap();
//! assert_eq!(result.docs.len(), 5);
//! let doc = &result.docs[0];
//! assert_eq!(doc["class"], "mammal");
//! ```


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate hyper;

pub mod http;
pub mod errors;

mod database;
pub use database::database;
