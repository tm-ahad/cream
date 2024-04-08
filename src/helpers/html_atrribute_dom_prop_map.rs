use std::collections::BTreeMap;
use serde_json::Value;
use ureq::Agent;
use crate::std_err::{ErrType, StdErr};

pub fn html_attribute_dom_prop_map() -> BTreeMap<String, Value> {
    let agent = Agent::new();
    let response = agent.get("https://raw.githubusercontent.com/tm-ahad/cream/master/ext/html_attr_dom_prop_map.json").call();

    if response.is_err() {
        StdErr::exec(
            ErrType::NetError,
            "https://raw.githubusercontent.com/tm-ahad/cream/master/ext/html_attr_dom_prop_map.json not found"
        );
        todo!()
    }

    let content = response.unwrap().into_string().unwrap();

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
