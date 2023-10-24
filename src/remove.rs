pub fn remove(script: &mut String) {
    while let Some(i) = script.find("@remove") {
        let mut end = i + 7;

        while &script[end..end + 1] != ";" {
            end += 1;
        }

        let mut idx = i + 7;

        while &script[idx..idx + 1] != " " {
            idx += 1;
        }

        let meth_removed_by = match script[i + 8..idx].trim() {
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
