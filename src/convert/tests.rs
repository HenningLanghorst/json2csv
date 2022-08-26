use crate::convert::convert_input_to_csv;

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

    let result = convert_input_to_csv(Box::new(json.as_bytes()), ',');

    println!("{:?}", result);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        r#"first_name,last_name,married,additional_information,number_of_children,address.street,address.city
Max,Power,false,,,,
Homer J.,Simpson,true,Some additional information,3,"742, Evergreen Terrace",Springfield
"#
    );
}
