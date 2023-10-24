use std::collections::BTreeMap;

pub fn html_attribute_dom_prop_map() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();

    map.insert("class", "className");
    map.insert("tabindex", "tabIndex");

    map
}
