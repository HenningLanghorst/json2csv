use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

#[cfg(test)]
mod tests;

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

pub fn convert_input_to_csv(input: &str, delimiter: char) -> Result<String, Box<dyn Error>> {
    match serde_json::from_str(input) {
        Ok(Value::Object(obj)) if obj.len() == 1 => {
            if let Some((_, Value::Array(items))) = obj.iter().next() {
                return convert_json_objects_to_csv(items, delimiter);
            }
        }
        Ok(Value::Array(items)) => {
            return convert_json_objects_to_csv(&items, delimiter);
        }
        _ => {}
    }

    let items_from_lines: Vec<Value> = input
        .lines()
        .filter_map::<Value, _>(|line| match serde_json::from_str::<Value>(line) {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("Cannot parse line '{}' ({})", line, e);
                None
            }
        })
        .collect();

    convert_json_objects_to_csv(&items_from_lines, delimiter)
}

fn convert_json_objects_to_csv(items: &[Value], delimiter: char) -> Result<String, Box<dyn Error>> {
    let (headers, rows) = extract_csv_data(items);
    create_csv_output(headers, rows, delimiter)
}

fn extract_csv_data(items: &[Value]) -> (Vec<String>, Vec<HashMap<String, String>>) {
    let mut headers = Headers::new();
    let mut rows = vec![];

    for item in items {
        if let Value::Object(fields) = item {
            let mut row = HashMap::new();
            process_json_fields("".to_string(), &mut headers, fields, &mut row);
            rows.push(row);
        }
    }
    (headers.vec, rows)
}

fn process_json_fields(
    prefix: String,
    headers: &mut Headers<String>,
    fields: &Map<String, Value>,
    row: &mut HashMap<String, String>,
) {
    for key in fields.keys() {
        let composite_key = format!("{}{}", prefix, key);
        let value = fields.get(key);
        match value {
            Some(Value::String(string)) => {
                headers.insert(composite_key.to_owned());
                row.insert(composite_key.to_owned(), string.to_owned());
            }
            Some(Value::Null) => {
                headers.insert(composite_key.to_owned());
                row.insert(composite_key.to_owned(), "".to_owned());
            }
            Some(Value::Object(map)) => {
                process_json_fields(format!("{}.", composite_key), headers, map, row)
            }
            Some(value) => {
                headers.insert(composite_key.to_owned());
                row.insert(composite_key.to_owned(), value.to_string());
            }
            None => {
                headers.insert(composite_key.to_owned());
                row.insert(composite_key.to_owned(), "".to_owned());
            }
        }
    }
}

fn create_csv_output(
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    delimiter: char,
) -> Result<String, Box<dyn Error>> {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(u8::try_from(delimiter)?)
        .from_writer(Vec::new());

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
