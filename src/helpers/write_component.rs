use crate::{component::{Component, cream_dom_name}, helpers::javascript::{javascript_function::javascript_function, javascript_function_call::javascript_function_call}};
use crate::helpers::javascript::javascript_assign::javascript_assign;

pub fn write_component(comp: Component) -> String {
    let rendered_html = comp.html_rendering_script().map_err(|e| format!("roxmltree: {e}")).unwrap();
    let script = format!("{}{}{}",
        format!("{};{}", comp.script, comp.dom_script),
        rendered_html.0,
        javascript_function_call("this.appendChild", vec![cream_dom_name(rendered_html.1)]),
    );
    
    javascript_assign(
        format!("Element.prototype.{}", comp.name),
        javascript_function(String::new(), script, vec!["params={}".to_string()])
    )
}
