use http::{post, get};
use serde_json;
use errors::Error;
use std::collections::HashMap;
use std::collections::BTreeMap;


pub struct Mango {
    url: String

}

#[derive(Serialize)]
struct NewIndex<'a> {
    #[serde(rename = "type")]
    json_type: &'a str,
    index: HashMap<&'a str, Vec<&'a str>>
}

/*
"{
 "ddoc": null,
 "name": "_all_docs",
 "type": "special",
 "def": {
  "fields": [
   {
    "_id": "asc"
   }
  ]
 }
}"
*/

#[derive(Deserialize)]
pub struct IndexDef {
    pub fields: Vec<BTreeMap<String, String>>
}

#[derive(Deserialize)]
pub struct IndexDoc {
    pub ddoc: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub def: IndexDef
}

#[derive(Deserialize)]
pub struct IndexList {
    pub total_rows: u32,
    pub indexes: Vec<IndexDoc>
}

impl Mango {
    fn new<S>(url: S) -> Mango where S: Into<String> {
       Mango { url: url.into() }
    }

    pub fn create_index(self, fields: &[&str]) -> Result<bool, Error> {
        let mut index_fields: HashMap<&str, Vec<&str>> = HashMap::new();
        index_fields.insert("fields", fields.to_vec());
        let index = NewIndex {
            json_type: "json",
            index: index_fields
        };

        let body = match serde_json::to_string(&index) {
            Ok(json_str) => json_str,
            Err(err) => return Err(Error::from(err))
        };

        match post(&format!("{}/_index", self.url), &body) {
            Ok(res) => Ok(true),
            Err(err) => return Err(Error::from(err))
        }
    }

    pub fn list_indexes(self) -> Result<IndexList, Error> {
        let resp = match get(&format!("{}/_index", self.url)) {
            Ok(res) => res,
            Err(err) => return Err(Error::from(err))
        };

        match serde_json::from_str::<IndexList>(&resp) {
            Ok(ind) => Ok(ind),
            Err(err) => Err(Error::from(err))
        }
    }

}



pub fn database (url: &str) -> Mango {
    Mango::new(url)
}
