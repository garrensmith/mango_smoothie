
//use serde_json;

// macro_rules! query {
//     ($($key:expr => $val:tt),*) => {
//         $(
//             println!("e {:?} {:?}", $key, $val);
//         )*
//     };
//
// }

/*
{
  "selector": {
    "year": {
      "$gt": 2010
    }
  },
  "fields": ["_id", "_rev", "year", "title"],
  "sort": [{"year": "asc"}],
  "limit": 10,
  "skip": 0
}

*/

// macro_rules! query {
//      ({$($section:expr => {$($key:expr => {$comp:expr => $val:expr}),*}),*}) => {{
//         $(
//             $(
//                 println!("out ${:?} {:?} {:?} {:?}", $section, $key, $comp, $val);
//             )*
//         )*
//     }};
// }

#[macro_export]
macro_rules! query_type {
    ({
        "selector" => {
            $(
                $key:expr => {
                    $comp:expr => $val:expr
                 }
             ),*
         }
    }) => {{
        let mut selector = Map::new();
        $(
            let mut kv = Map::new();
            kv.insert($comp.to_string(), serde_json::to_value($val));
            selector.insert($key.to_string(), kv);
            println!("selector {:?} {:?} {:?}", $key, $comp, $val);
        )*
        serde_json::to_value(selector)
    }};

    ({
        "fields" => $fields:expr
    }) => {{
        println!("field {:?}", $fields);
        serde_json::to_value($fields.to_vec())
    }};

    ({
        "sort" => [$({$key:expr => $sort:expr}),*]
    }) => {{
        let mut sort = Map::new();
        $(
            sort.insert($key.to_string(), $sort.to_string());
        )*
        serde_json::to_value(sort)
    }};

    ({
        $section:expr => $fields:expr
    }) => {{
        println!("section limit {:?} {:?}", $section, $fields);
        serde_json::to_value($fields)
    }};
}


#[macro_export]
macro_rules! query {
    ( {$($section:tt => $content:tt),*} ) => {{
        extern crate serde_json;
        pub type Map<K,V> = serde_json::Map<K, V>;
        //use serde_json;
        //use serde_json::{Map};
        let mut map = Map::new();
        $(
            println!("section {:?}", $section);
            map.insert($section.to_string(), query_type!({$section => $content}));
        )*
        map
    }};
}

#[cfg(test)]
mod tests {
    use serde_json;
    use serde_json::{Map};

    #[test]
    fn create_selector_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$eq" => "boom"
                },
                "name" => {
                    "$lte" => "garren"
                }
            }
        });

        let map: Map<String, Map<String, String>> = serde_json::from_value(query.get("selector").unwrap().clone()).unwrap();
        let _id = map.get("_id").unwrap();
        let name = map.get("name").unwrap();

        assert_eq!(_id.get("$eq").unwrap().to_string(), "boom");
        assert_eq!(name.get("$lte").unwrap().to_string(), "garren");
    }

    #[test]
    fn set_fields_in_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                }
            },
            "fields" => ["_id", "name"]
        });

        let fields: Vec<String> = serde_json::from_value(query.get("fields").unwrap().clone()).unwrap();
        assert_eq!(fields[0], "_id");
        assert_eq!(fields[1], "name");
    }

    #[test]
    fn set_limit_in_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                }
            },
            "fields" => ["_id", "name"],
            "limit" => 10
        });

        let limit: i32 = serde_json::from_value(query.get("limit").unwrap().clone()).unwrap();
        assert_eq!(limit, 10);
    }

    #[test]
    fn set_skip_in_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                }
            },
            "fields" => ["_id", "name"],
            "skip" => 3
        });

        let skip: i32 = serde_json::from_value(query.get("skip").unwrap().clone()).unwrap();
        assert_eq!(skip, 3);
    }

    #[test]
    fn set_sort_in_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                }
            },
            "fields" => ["_id", "name"],
            "sort" => [{"_id" => "asc"}]
        });

        let sort: Map<String, String> = serde_json::from_value(query.get("sort").unwrap().clone()).unwrap();
        let _id = sort.get("_id").unwrap();
        assert_eq!(_id, "asc");
    }

    #[test]
    fn creates_correct_query_string () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                },
                "name" => {
                    "lte" => "bob"
                }
            },
            "fields" => ["_id", "name"],
            "limit" => 10,
            "skip" => 3,
            "sort" => [{"_id" => "asc"}]
        });

        let query_string = serde_json::to_string(&query).unwrap();
        println!("qs {:?}", query_string);
        assert_eq!(query_string, "{\"fields\":[\"_id\",\"name\"],\"limit\":10,\"selector\":{\"_id\":{\"$gt\":\"1\"},\"name\":{\"lte\":\"bob\"}},\"skip\":3,\"sort\":{\"_id\":\"asc\"}}");
    }
}
