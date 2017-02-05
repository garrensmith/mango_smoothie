# Mango Smoothie [![Build Status](https://travis-ci.org/garrensmith/mango_smoothie.svg?branch=master)](https://travis-ci.org/garrensmith/mango_smoothie)

A CouchDB Mango/Cloudant query client for rust. It supports creating indexes, listing indexes and querying indexes

For (docs)[http://garrensmith.com/mango_smoothie/]

```
extern crate mango_smoothie;
use mango_smoothie::database;
#[macro_use]
extern crate serde_json;


let query_resp = db.query_index(json!({
                "selector": {
                    "diet": {
                        "$eq": "omnivore"
                    }
                },
                "fields": ["_id", "_rev", "name", "class", "diet"]
             }));

  let result = query_resp.unwrap();
  let doc = &result.docs[0];
  assert_eq!(doc.get("class").unwrap().as_str().unwrap(), "mammal");
```

## License
*Mango Smoothie* is licensed under MIT. See the LICENSE file for more
