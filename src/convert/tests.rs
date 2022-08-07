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
    "additional_information": "Some additional information",
    "address": {
      "street": "742, Evergreen Terrace",
      "city": "Springfield"
    }
  }
]"#;

    let value: Value = serde_json::from_str(json).unwrap();
    let result = convert(value);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        r#"first_name,last_name,married,additional_information,number_of_children,address.street,address.city
Max,Power,false,,,,
Homer J.,Simpson,true,Some additional information,3,"742, Evergreen Terrace",Springfield
"#
    );
}
