pub fn dom_element(name: &str) -> String {
    if name == "template" {
        format!("document.createDocumentFragment()")
    } else {
        format!("document.createElement({name:?})")
    }
}
