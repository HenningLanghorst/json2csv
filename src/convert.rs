use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Headers<T> {
    vec: Vec<T>,
    set: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Headers<T> {
    fn new() -> Headers<T> {
        Headers {
            vec: vec![],
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, value: T) {
        if self.set.insert(value.clone()) {
            self.vec.push(value);
        }
    }
}

pub fn convert(json: Value) -> Result<String, Box<dyn Error>> {
    match json {
        Value::Array(items) => {
            let (headers, rows) = process_array_items(items);
            create_csv(headers, rows)
        }
        _ => Ok("".to_string()),
    }
}

fn process_array_items(items: Vec<Value>) -> (Vec<String>, Vec<HashMap<String, String>>) {
    let mut headers = Headers::new();
    let mut rows = vec![];

    for item in items {
        if let Value::Object(map) = item {
            let mut row = HashMap::new();
            for key in map.keys() {
                headers.insert(key.to_owned());

                let string = map.get(key).map(value_to_string).unwrap_or_default();
                row.insert(key.to_owned(), string);
            }
            rows.push(row);
        }
    }
    (headers.vec, rows)
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(string) => string.to_owned(),
        Value::Null => "".to_owned(),
        _ => value.to_string(),
    }
}

fn create_csv(
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
) -> Result<String, Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(&headers)?;
    for row in rows {
        let record: Vec<String> = headers
            .iter()
            .map(|header| row.get(header).map(String::to_owned).unwrap_or_default())
            .collect();
        writer.write_record(&record)?;
    }

    let bytes = writer.into_inner()?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
}

#[cfg(test)]
mod tests {
    use crate::convert::convert;
    use serde_json::Value;

    #[test]
    fn json_array() {
        let json = r#"[
  {
    "first_name": "Max",
    "last_name": "Power",
    "married": false,
    "additional_information": null
  },
  {
    "first_name": "Homer J.",
    "last_name": "Simpson",
    "number_of_children": 3,
    "married": true,
    "additional_information": "Some additional information"
  }
]"#;

        let value: Value = serde_json::from_str(json).unwrap();
        let result = convert(value);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"first_name,last_name,married,additional_information,number_of_children
Max,Power,false,,
Homer J.,Simpson,true,Some additional information,3
"#
        );
    }
}
