use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;

pub fn remove(script: &mut String, f_name: &str) {
    while let Some(i) = script.find("@remove:") {
        let end = read_until(&script, i+7, ";", f_name, ComponentPart::Unknown);
        let idx = read_until(&script, i+7, " ", f_name, ComponentPart::Unknown);

        let meth_removed_by = match script[i + 9..idx].trim() {
            "class" => "getElementByClassName",
            "id" => "getElementById",
            "tag" => "getElementByTagName",
            "name" => "getElementByName",
            n => panic!("Cannot find element by {n}"),
        };

        let k = &script[idx + 1..end];

        script.push_str(&format!("document.{meth_removed_by}({k}).remove()"))
    }
}
