
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

macro_rules! query_type {
    ({
        $section:expr => {
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
        $field_section:expr => $fields:expr
    }) => {{
        let v = vec!["_boom"];
        let mut fields = Map::new();
        println!("field {:?}", $fields);
        fields.insert($field_section.to_string(), serde_json::to_value(v));
        serde_json::to_value(fields)
    }};
}

#[macro_export]
macro_rules! query {
    ( {$($section:expr => $content:tt),*} ) => {{
        use serde_json;
        use serde_json::{Map};
        let mut map = Map::new();
        $(
            println!("section {:?}", $section);
            map.insert($section.to_string(), query_type!({$section => $content}));
        )*
        map
    }};
}



// fn main() {
//     let b = query!({
//         "selector" => {
//             "_id" => {
//                 "$eq" => "boom"
//             },
//             "name" => {
//                 "$lte" => "garren"
//             }
//         }//,
//         //"fields" => ["_id", "_rev"] //tt
//     });
//
//     println!("B {:?}", b);
// }

/*
doc! { "title" => "Jaws",
    "array" => [ 1, 2, 3 ] };


#[macro_export]
macro_rules! doc {
    () => {{ $crate::Document::new() }};

    ( $($key:expr => $val:tt),* ) => {{
        let mut document = $crate::Document::new();

        $(
            document.insert_bson($key.to_owned(), bson!($val));
        )*

        document
    }};
}

*/

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
    fn set_keys_in_query () {
        let query = query!({
            "selector" => {
                "_id" => {
                    "$gt" => "1"
                }
            },
            "fields" => ["_id", "name"]
        });

        //assert_eq!(query.get("fields").unwrap()[0], "_id");
        //assert_eq!(query.get("fields").unwrap()[1], "name");
    }
}
