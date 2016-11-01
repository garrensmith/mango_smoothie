 #![feature(proc_macro)]
//
extern crate hyper;
 #[macro_use]
 extern crate serde_derive;
 extern crate serde;
 extern crate serde_json;
//
use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType};
use std::collections::HashMap;


// #[derive(Deserialize)]
// pub struct Vendor {
//     name: String
// }

// #[derive(Deserialize)]
// pub struct Welcome {
//     couchdb: String,
//     version: String,
//     //vendor: Vendor
//     vendor: HashMap<String, String>
// }
//{"couchdb":"Welcome","version":"ac3dae3","vendor":{"name":"The Apache Software Foundation"}}

/*
{
  "selector": {
    "_id": {
      "$gt": null
    }
  }
}

    find().selector().field
    Selector([field("_id", GreaterThan(5), LessThan(2))]);

*/

use std::fmt;

pub struct QueryBuilder {
    field: String,
    operations: HashMap<String, String>
}

impl QueryBuilder {
    pub fn new(field: &str) -> QueryBuilder {
        println!("created query");
        QueryBuilder {
            operations: HashMap::new(),
            field: field.to_string()
        }
    }

    pub fn gt<'a, T>(&'a mut self, value: T) -> &'a mut QueryBuilder where T: fmt::Display {
        self.operations.insert("$gt".to_string(), value.to_string());
        self
    }

    pub fn lte<'a, T>(&'a mut self, value: T) -> &'a mut QueryBuilder where T: fmt::Display {
        self.operations.insert("$lte".to_string(), value.to_string());
        self
    }
}

impl fmt::Display for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "({}, boom wow {})", self.x, self.y)
        write!(f, "{{");
        write!(f, "{}:{{", self.field);
        for (key, value) in self.operations.iter() {
            write!(f, "{}:{},", key, value);
        }
        write!(f, "}}");
        write!(f, "}}")
    }
}

pub struct Find {
    selector: vec<>
}

impl Find {
    pub fn new () -> Find {
        Find { selector: vec::new()}
    }

    fn find (&self, body: &str) {
        let client = Client::new();
        // let mut headers = Headers::new();
        // headers.set(
        //     ContentType::json()
        // );
        println!("BODY {:?}", serde_json::to_string(&body).unwrap());
        let mut res = client
                        .post("http://tester:testerpass@dev:5984/animaldb/_find")
                        //.headers(headers)
                        .header(ContentType::json())
                        .body(&serde_json::to_string(&body).unwrap())
                        .send()
                        .unwrap();

        // Read the Response.
        let mut resp = String::new();
        res.read_to_string(&mut resp).unwrap();
        println!("QUERY {}", resp);
    }

    fn selector (&mut self, query: QueryBuilder) -> &mut Find {
        self.selector = query.to_string();
        self
    }

    pub fn run (&self) {
        self.find(&self.to_string());
    }
}

impl fmt::Display for Find {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{");
        write!(f, "selector: {{");
        write!(f, "{}", self.selector);
        write!(f, "}}");
        write!(f, "}}")
    }
}

fn query(field: &str) -> QueryBuilder {
    QueryBuilder::new(&field)
}

fn find() -> Find {
    Find {
        selector: "".to_string()
    }
}

fn main () {
    // let mut buf = String::new();
    // let client = Client::new();
    // let mut res = client.get("http://127.0.0.1:5984").send().unwrap();
    // res.read_to_string(&mut buf).unwrap();
    // match serde_json::from_str::<Welcome>(&buf) {
    //     Ok(welcome) => println!("{} version {} with vendor {}", welcome.couchdb, welcome.version, welcome.vendor.get("name").unwrap()),
    //     _ => println!("errrr")
    // };
    // println!("res {}", buf);
    let mut builder = query("_id");
    builder.gt(5).lte(2);
    println!("output {}", builder);

    find()
        .selector(builder)
        .run();

}
