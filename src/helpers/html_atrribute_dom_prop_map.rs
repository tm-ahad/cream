use crate::std_err::{ErrType, StdErr};
use std::collections::BTreeMap;
use serde_json::Value;
use tinyget::get;

pub fn html_attribute_dom_prop_map() -> BTreeMap<String, Value> {
    let response = get("https://raw.githubusercontent.com/tm-ahad/cream/master/ext/html_attr_dom_prop_map.json").send();

    if response.is_err() {
        StdErr::exec(
            ErrType::NetError,
            "https://raw.githubusercontent.com/tm-ahad/cream/master/ext/html_attr_dom_prop_map.json not found"
        );
        todo!()
    }

    let binding = response.unwrap();
    let content = binding.as_str().unwrap();

    let json: Value = serde_json::from_str(&content).unwrap();

    if let Value::Object(map) = json {
        map.into_iter().collect()
    } else {
        StdErr::exec(
            ErrType::NetError,
            "Cannot parse json!"
        );
        todo!()
    }
}
