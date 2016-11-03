extern crate rustybench;

use rustybench::{database};

#[test]
fn lists_index () {
    match database("http://tester:testerpass@127.0.0.1:5984/animaldb")
               .list_indexes() {
                   Ok(indexes) => {
                       assert!(indexes.total_rows > 0);
                       assert_eq!(indexes.indexes[0].name, "_all_docs".to_string());
                       assert!(indexes.indexes[0].def.fields[0].contains_key(&"_id".to_string()));
                   }
                   Err(e) =>  panic!("Should not get here {}", e)
               };

}
