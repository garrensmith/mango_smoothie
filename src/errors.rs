use std::fmt;
use hyper::Error as HyperError;
use std::error::Error as StdError;
use std::io;
use std::collections::HashMap;
use serde_json;
use serde_json::Error as JsonError;

use self::Error::{
    Json,
    Http,
    Io,
    Query
};

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Http(HyperError),
    Io(io::Error),
    Query {
        error: String,
        description: String
    }
}

impl Error {
    pub fn from_couch(msg: &str) -> Error {
        let convert = match serde_json::from_str::<HashMap<String, String>>(&msg) {
            Ok(map) => map,
            Err(e) => return Json(e)
        };

        let err = match convert.get("error") {
            Some(msg) => msg.to_string(),
            None => "Unknown".to_string()
        };

        let description = match convert.get("reason") {
            Some(msg) => msg.to_string(),
            None => "unknown reason".to_string()
        };

        Query {
            error: err,
            description: description
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Json(ref e) => write!(f, "{}", e),
            Http(ref e) => write!(f, "{}", e),
            Io(ref e) => write!(f, "{}", e),
            Query{ref error, ref description} => write!(f, "{}:{}", error, description)
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Json( ref e ) => e.description(),
            Http( ref e ) => e.description(),
            Io( ref e ) => e.description(),
            Query { .. } => "CouchDB responded with an error"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Json(ref e) => Some(e),
            Http(ref e) => Some(e),
            Io(ref e) => Some(e),
            Query{ .. } => None
        }
    }
}


impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Http(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Json(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Error::*;

    #[test]
    fn from_couch_warns_on_json_parse_error () {
        match Error::from_couch("This will not parse") {
            Json(err) => assert!(true),
            _ => panic!("I was expecting a Json error!")
        }
    }

    #[test]
    fn from_couch_returns_parsed_json_error () {
        let str = "{\"error\":\"not_found\",\"reason\":\"Database does not exist.\"}";
        match Error::from_couch(str) {
            Query {ref error, ..} => assert_eq!(error, "not_found"),
            _ => panic!("I was expecting query error")

        }
    }

    #[test]
    fn from_couch_works_with_different_hashmap_values () {
        let str = "{\"oops\":\"not_found\",\"what_happened\":\"Database does not exist.\"}";
        match Error::from_couch(str) {
            Query {ref error, ref description} => {
                assert_eq!(error, "Unknown");
                assert_eq!(description, "unknown reason");
            }
            _ => panic!("I was expecting query error")
        }
    }
}
