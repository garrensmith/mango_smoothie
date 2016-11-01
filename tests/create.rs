// extern crate rustybench;
//
// use rustybench::{database, create_index};
//
// #[test]
// fn creates_index () {
//     let resp = database("http://127.0.0.1:5984/animaldb")
//     .create_index(["class", "diet"])
//
//     assert!(resp.is_ok());
// }

// #[test]
// fn create_index_again_warns () {
//     let db = database("http://127.0.0.1:5984/animaldb");
//
//     let resp = db.create_index(["class"]);
//     assert!(resp.is_ok());
//
//     let resp2 = db.create_index(["class"]);
// }
