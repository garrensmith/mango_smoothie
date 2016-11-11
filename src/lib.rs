#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//
// use std::collections::HashMap;
// use std::fmt;

extern crate hyper;

pub mod http;
pub mod errors;
pub mod query;
mod database;
pub use database::database;
// use http::post;
//
//
// #[derive(Serialize)]
// pub struct QueryBuilder {
//     field: String,
//     operations: HashMap<String, String>
// }
//
// impl QueryBuilder {
//     pub fn new(field: &str) -> QueryBuilder {
//         QueryBuilder {
//             operations: HashMap::new(),
//             field: field.to_string()
//         }
//     }
//
//     pub fn gt<'a, T>(&'a mut self, value: T) -> &'a mut QueryBuilder where T: fmt::Display {
//         self.operations.insert("$gt".to_string(), value.to_string());
//         self
//     }
//
//     pub fn lte<'a, T>(&'a mut self, value: T) -> &'a mut QueryBuilder where T: fmt::Display {
//         self.operations.insert("$lte".to_string(), value.to_string());
//         self
//     }
// }
//
// #[derive(Deserialize)]
// pub struct QueryResponse {
//     pub docs: Vec<HashMap<String, String>>
// }
//
// #[derive(Serialize)]
// pub struct Find {
//     #[serde(rename = "selector")]
//     query: HashMap<String, HashMap<String,String>>
//     //sort
//     //fields
// }
//
// impl Find {
//     pub fn new () -> Find {
//         Find {
//             query: HashMap::new()
//         }
//     }
//
//     //possibly make this an array of query builders
//     pub fn selector (&mut self, query: QueryBuilder) -> &mut Find {
//         self.query.insert(query.field, query.operations);
//         self
//     }
//
//     pub fn to_json (&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
//
//     pub fn run (&self) -> QueryResponse {
//         post("http://tester:testerpass@dev:5984/animaldb/_find", &self.to_json());
//
//         serde_json::from_str(&resp).unwrap()
//     }
// }
//
// //#[cfg(test)]
// //mod tests {
//     #[test]
//     fn create_field_selector () {
//         let mut query = QueryBuilder::new("_id");
//         query.gt(5);
//         assert_eq!(query.field, "_id".to_string());
//     }
//
//     #[test]
//     fn create_operator () {
//         let mut query = QueryBuilder::new("_id");
//         query.gt(5);
//         assert_eq!(query.operations.get("$gt").unwrap(), "5");
//     }
//
//     #[test]
//     fn create_new_find () {
//         let find = Find::new();
//         assert_eq!(find.query.len(), 0);
//     }
//
//     #[test]
//     fn create_find_with_query () {
//         let mut query = QueryBuilder::new("_id");
//         query.gt(5);
//         let mut find = Find::new();
//         find.selector(query);
//         assert_eq!(find.query.len(), 1);
//     }
//
//     #[test]
//     fn convert_selector_to_json () {
//         let mut query = QueryBuilder::new("_id");
//         query.gt(5);
//         let mut find = Find::new();
//         find.selector(query);
//         println!("wow!!!! {:?}", find.to_json());
//         assert_eq!("{\"selector\":{\"_id\":{\"$gt\":\"5\"}}}", find.to_json());
//     }
//
//     #[test]
//     fn runs_find_returns () {
//         let mut query = QueryBuilder::new("_id");
//         query.gt(5);
//         let mut find = Find::new();
//         find.selector(query);
//         let out = find.run();
//         println!("docs {:?}", out.docs);
//     }
// //}
