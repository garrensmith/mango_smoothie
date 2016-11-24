use http::{post, get};
use serde_json;
use serde_json::{Map, Value};
use errors::Error;
use std::collections::HashMap;
use std::collections::BTreeMap;
use hyper::Url;



pub struct Mango {
    url: Url
}

#[derive(Serialize)]
pub struct NewIndex<'a> {
    #[serde(rename = "type")]
    json_type: &'a str,
    index: HashMap<&'a str, Vec<&'a str>>
}

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

#[derive(Deserialize)]
pub struct QueryResult {
    pub warning: Option<String>,
    pub docs: Vec<BTreeMap<String, serde_json::Value>>
}

impl Mango {
    fn new<S>(url_str: S) -> Result<Mango, Error> where S: Into<String> {
        match Url::parse(&url_str.into()) {
            Ok(url) => Ok(Mango { url: url }),
            Err(err) => Err(Error::from(err))
        }
    }

    pub fn create_index(&mut self, fields: &[&str]) -> Result<bool, Error> {
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

        println!("self.url {:?}", self.url);

        let url = try!(Url::parse(&format!("{}/_index", self.url.to_string())));
        try!(post(&url, &body));
        Ok(true)
    }

    pub fn list_indexes(&mut self) -> Result<IndexList, Error> {
        let url = try!(Url::parse(&format!("{}/_index", self.url.to_string())));
        let resp = try!(get(&url));

        match serde_json::from_str::<IndexList>(&resp) {
            Ok(ind) => Ok(ind),
            Err(err) => Err(Error::from(err))
        }
    }

    pub fn query_index(&mut self, query: Map<String, Value>) -> Result<QueryResult, Error> {
        let url = try!(Url::parse(&format!("{}/_find", self.url.to_string())));

        let body = match serde_json::to_string(&query) {
            Ok(json_str) => json_str,
            Err(err) => return Err(Error::from(err))
        };

        println!("self.url {:?} {:?}", self.url, body);
        let resp = try!(post(&url, &body));
        let result = try!(serde_json::from_str::<QueryResult>(&resp));
        Ok(result)
    }

}


/// The entry point for each Mango Smoothie request
pub fn database (url: &str) -> Result<Mango, Error> {
    Mango::new(url)
}
