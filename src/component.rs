use rand::Rng;
use roxmltree::Node;
use roxmltree::Error;
use crate::helpers::javascript::javascript_init_var::javascript_init_var;
use crate::helpers::javascript::javascript_function_call::javascript_function_call;
use crate::helpers::javascript::javascript_assign::javascript_assign;
use crate::helpers::javascript::javascript_string::javascript_string;

#[derive(Debug)]
pub struct Component {
    pub html: String,
    pub script: String,
    pub dom_script: String,
    pub name: String,
}

pub fn cream_dom_name(id: u64) -> String {format!("cream_element{id}")}

impl Component {
    pub fn new(
        script: String,
        html: String,
        name: String,
        dom_script: String
    ) -> Self {
        Component {
            script,
            html,
            name,
            dom_script
        }
    }

    pub fn html_rendering_script(&self) -> Result<(String, u64), Error>  {
        let res = roxmltree::Document::parse(&self.html)?;
        Ok(self.rendering_script_from_desc(res.root_element()))
    }

    fn rendering_script_from_desc(&self, node: Node) -> (String, u64) {
        let mut rng = rand::rng();
        let mut script: String = String::new();

        let root_u64_id = rng.next_u64();
        let root_id = &cream_dom_name(root_u64_id);
        script.push_str(&javascript_init_var(
            root_id,
            &javascript_function_call("document.createElement", vec![javascript_string(node.tag_name().name())])
        ));
        
        for attr in node.attributes() {
            if attr.name() == "render" {
                script.push_str(&javascript_assign("self".to_string(), root_id.clone()));
                script.push_str(attr.value());
            } else if attr.name().starts_with("on_") {
                let event = attr.name()[3..].to_string();
                script.push_str(&javascript_function_call(
                    &format!("{}.addEventListener", root_id), 
                    vec![javascript_string(&event), attr.value().to_string()]
                ));
            } else {
                let value = if attr.value().starts_with('@') {
                    attr.value()[1..].to_string()
                } else {
                    javascript_string(attr.value())
                };

                script.push_str(&javascript_assign(
                    format!("{}.{}", root_id, attr.name()),
                    value
                ));
            }
        }

        for child in node.children() {
            if child.is_text() {
                let text = child.text().unwrap();
                let value = if text.starts_with('@') {
                    text[1..].to_string()
                } else {
                    javascript_string(text)
                };

                script.push_str(&format!("{}.textContent={};", root_id, value));
            } else {
                let rendered_child = self.rendering_script_from_desc(child);
                script.push_str(&format!("{};", rendered_child.0));
                script.push_str(&format!("{}.appendChild({});", root_id, &cream_dom_name(rendered_child.1)));
            }
        }
        
        (script, root_u64_id)
    }
}

impl Clone for Component {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            script: self.script.clone(),
            html: self.html.clone(),
            dom_script: self.dom_script.clone()
        }
    }
}
