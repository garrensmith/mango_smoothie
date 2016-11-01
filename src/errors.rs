use std::fmt;
use hyper::Error as hyperError;
use std::error::Error as StdError;
use std::collections::HashMap;
use serde_json;

#[derive(Debug)]
pub struct Error {
    pub error: String,
    pub description: String

}

impl Error {
    pub fn from_couch(msg: &str) -> Error {
        let convert = match serde_json::from_str::<HashMap<String, String>>(&msg) {
            Ok(map) => map,
            _ => return Error {
                error: "CouchDB error".to_string(),
                description: "Invalid JSON received".to_string()
            }
        };

        let err = match convert.get("error") {
            Some(msg) => msg.to_string(),
            None => "Unknown".to_string()
        };

        let description = match convert.get("reason") {
            Some(msg) => msg.to_string(),
            None => "unknown reason".to_string()
        };

        Error {
            error: err,
            description: description
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.error, self.description)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.description
    }

    //NOT sure what to do here!!!
    fn cause(&self) -> Option<&StdError> {
        //Some(&self.error)
        None
    }
}


impl From<hyperError> for Error {
    fn from(err: hyperError) -> Error {
        Error {
            error: "request error".to_string(),
            description: err.to_string()
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error {
            error: "not_found".to_string(),
            description: err.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_couch_warns_on_json_parse_error () {
        let error = Error::from_couch("This will not parse");
        assert_eq!(error.error, "CouchDB error".to_string());
    }

    #[test]
    fn from_couch_returns_parsed_json_error () {
        let str = "{\"error\":\"not_found\",\"reason\":\"Database does not exist.\"}";
        let error = Error::from_couch(str);
        assert_eq!(error.error, "not_found".to_string());
    }

    #[test]
    fn from_couch_works_with_different_hashmap_values () {
        let str = "{\"oops\":\"not_found\",\"what_happened\":\"Database does not exist.\"}";
        let error = Error::from_couch(str);
        assert_eq!(error.error, "Unknown".to_string());
        assert_eq!(error.description, "unknown reason".to_string());
    }

}
