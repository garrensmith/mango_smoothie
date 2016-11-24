# Mango Smoothie [![Build Status](https://travis-ci.org/garrensmith/mango_smoothie.svg?branch=master)](https://travis-ci.org/garrensmith/mango_smoothie)

A CouchDB Mango/Cloudant query client for rust. It supports creating indexes, listing indexes and querying indexes

For (docs)[https://docs.rs/mango_smoothie/]

```
#[macro_use]
extern crate mango_smoothie;
use mango_smoothie::{database};


let query_resp = db.query_index(query!({
                "selector" => {
                    "diet" => {
                        "$eq" => "omnivore"
                    }
                },
                "fields" => ["_id", "_rev", "name", "class", "diet"]
             }));

  let result = query_resp.unwrap();
  let doc = &result.docs[0];
  assert_eq!(doc.get("class").unwrap().as_str().unwrap(), "mammal");
```

## License
*Mango Smoothie* is licensed under MIT. See the LICENSE file for more
