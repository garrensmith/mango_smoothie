extern crate mango_smoothie;
use mango_smoothie::database;
#[macro_use]
extern crate serde_json;

#[test]
fn queries_index() {
    let mut db = database("http://tester:testerpass@127.0.0.1:5984/animaldb").unwrap();
    let resp = db.create_index(&["diet"]);
    assert!(resp.is_ok());

    let query_resp = db.query_index(json!({
                  "selector": {
                      "diet": {
                          "$eq": "omnivore"
                      }
                  },
                  "fields": ["_id", "_rev", "name", "class", "diet"]
               }));

    assert!(query_resp.is_ok());
    let result = query_resp.unwrap();
    assert_eq!(result.docs.len(), 5);
    let doc = &result.docs[0];
    //assert_eq!(doc.get("class").unwrap().as_str().unwrap(), "mammal");
    assert_eq!(doc["class"], "mammal");
}
