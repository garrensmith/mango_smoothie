// extern crate rustybench;
//
// use rustybench::{QueryBuilder, Find};
//
// #[test]
// fn runs_find_returns () {
//     let mut query = QueryBuilder::new("_id");
//     query.gt(5);
//     let mut find = Find::new();
//     find.selector(query);
//     let out = find.run();
//     println!("docs {:?}", out.docs);
// }
