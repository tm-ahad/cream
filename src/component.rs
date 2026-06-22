use std::process::exit;

use oxc_diagnostics::OxcDiagnostic;
use roxmltree::Attribute;
use serde_json::Map;
use serde_json::Value;
use crate::consts::DOCUMENT_BODY;
use crate::helpers::dependancy_graph::DependancyGraph;
use crate::helpers::javascript::javascript_function::javascript_function;
use crate::helpers::javascript::javascript_init_var::EMPTY_OBJ;
use crate::helpers::javascript::javascript_init_var::NULL;
use crate::helpers::javascript::javascript_init_var::javascript_init_var;
use crate::helpers::javascript::javascript_function_call::javascript_function_call;
use crate::helpers::javascript::javascript_assign::javascript_assign;
use crate::helpers::javascript::javascript_string::javascript_string;
use crate::std_err::ErrType;
use crate::std_err::StdErr;
use roxmltree::Node;
use roxmltree::Error;
use rand::Rng;

#[derive(Debug)]
pub struct Component<'a> {
    pub router_map: &'a Map<String, Value>,
    pub html: String,
    pub name: String,
    pub out: String,
    pub dep_graph: &'a mut DependancyGraph
}

pub fn special_trim(s: String) -> String {
    s
        .lines()
        .map(|e| e.trim())
        .collect::<std::vec::Vec<&str>>()
        .join("\n")
}

pub fn cream_dom_name(id: u64) -> String {format!("cream_element{id}")}
pub fn cream_component(id: u64) -> String {format!("cream_component{id}")}
pub fn cream_object(id: u64) -> String {format!("cream_object{id}")}
pub fn std_lib_path(name: &str) -> String {format!("./.cream_std/{name}")}

#[derive(Default)]
pub struct RenderReturn {
    pub rendering_script: String,
    pub script: String,
    pub root_dom_id: u64,
    pub comp_name: String
}

fn preproc_attr_value(s: &str) -> String {
    format!("`{s}`")
} 

pub fn format_oxc_diag(diag: &OxcDiagnostic, src: String) -> String {
    format!("{} at {}:{}", diag.message, src, diag.labels.clone()[0].offset())
}

pub fn final_build_string(render: RenderReturn, comp_id: u64) -> String {
    special_trim(format!("
            let self; let onRender=function(){{}}; let elements = {{}};
            {}; function {}(params={{}}) {{{}; onRender();
                return {}
            }}; {}
            export {{{} as {}, mount}};
        ",  render.script, cream_component(comp_id),
            render.rendering_script,
            cream_dom_name(render.root_dom_id),
            javascript_function(String::from("mount"), 
                &format!(
                    "document.body.appendChild({}()); onRender()",
                    cream_component(comp_id)
                ),
                vec![]
            ),
            cream_component(comp_id),
            render.comp_name
        ))
}

impl<'a> Component<'a> {
    pub fn new(
        html: String,
        name: String,
        router_map: &'a Map<String, Value>,
        dep_graph: &'a mut DependancyGraph
    ) -> Self {
        Self {
            html,
            name,
            router_map,
            dep_graph,
            out: String::new()
        }
    }

    fn parse_render_value(&self, node: &Node, id: u64) -> (String, Vec<u64>) {
        let mut ret = String::new();
        let mut ids = vec![];

        for child in node.children().rev() {
            if child.is_text() || child.is_comment() {
                ret.insert_str(0,child.text().unwrap());
                continue;
            }

            let code = self.rendering_script_from_desc(child, DOCUMENT_BODY.to_string());
            ret.insert_str(0, &javascript_function_call(
                &format!("{}.appendChild", cream_dom_name(id)),
                vec![cream_dom_name(code.root_dom_id)]
            ));
            ret.insert_str(0, &code.rendering_script);
            ids.push(code.root_dom_id);
        }

        (ret, ids)
    }

    pub fn html_rendering_script(&self) -> Result<RenderReturn, Error>  {
        let synt = format!("<temp>{}</temp>", &self.html.trim());
        let res = roxmltree::Document::parse(&synt)?;
        Ok(self.rendering_script_from_desc(res.root_element(), DOCUMENT_BODY.to_string()))
    }

    fn subscribe_fn_name() -> String {"entangle".to_string()}

    fn rendering_script_from_desc(&self, node: Node, parent_id: String) -> RenderReturn {
        let mut rng = rand::rng();
        let mut rendering_script: String = String::new();
        let mut script = String::new();
        let mut comp_name = String::new();

        let root_u64_id = rng.next_u64();
        let root_id = &cream_dom_name(root_u64_id);

        let is_comp = node.tag_name().name().chars().next().unwrap().is_ascii_uppercase();
        let root_def = if is_comp {
            "undefined"
        } else {
            &format!("{};",
                &javascript_function_call("document.createElement", vec![javascript_string(node.tag_name().name())])
            )
        };

        let mut comp_attr_map_id = 0;
        if is_comp {
            comp_attr_map_id = rng.next_u64();
            rendering_script.push_str(
                &javascript_init_var(&cream_object(comp_attr_map_id), EMPTY_OBJ)
            );
        }

        rendering_script.push_str(&javascript_init_var(root_id,
            if is_comp {
                NULL
            } else {
                root_def
            }
        ));
        
        let mut render_self = if is_comp {
            javascript_assign(root_id, root_def)
        } else {
            format!(";{}.remove();{};",  root_id, javascript_assign(root_id, root_def))
        };

        let mut post_render_hooks = String::new();
        let rand =  rng.next_u64();
            
        for attr in node.attributes() {
            if is_comp {
                let value = &preproc_attr_value(attr.value());
                render_self.push_str(&javascript_assign(
                    &format!("{}.{}", cream_object(comp_attr_map_id), attr.name()),
                    &value
                ));
                continue;
            }

            if attr.name().starts_with("on_") {
                let event = attr.name()[3..].to_string();
                render_self.push_str(&javascript_function_call(
                    &format!("{}.addEventListener", root_id), 
                    vec![javascript_string(&event), attr.value().to_string()]
                ));
            } else if attr.name() == "key" {
                let value = preproc_attr_value(attr.value());
                render_self.push_str(&format!(";elements[{}] = {};", value, root_id));
            } else if attr.name() == "subscribe" {
                post_render_hooks.push_str(
                &format!(
                        ";for (let haaland of {}) {{haaland.{}(render_fn{rand})}};", attr.value(), 
                        Component::subscribe_fn_name()
                    )
                );
            } else {
                let value = preproc_attr_value(attr.value());
                render_self.push_str(&javascript_assign(
                    &format!("{}.{}", root_id, attr.name()),
                    &value
                ));
            }
        }

        for child in node.children() {
            if child.tag_name().name() == "script" {
                script = child.text().unwrap_or_default().to_string();
                let attr = child.attributes().filter(|attr| {
                    attr.name() == "name"
                }).collect::<Vec<Attribute>>();

                if attr.is_empty() {
                    StdErr::exec(ErrType::NotFound, &format!("\"name\" attribute of script tag in {}", self.name));
                    exit(1)
                } else {
                    comp_name = attr[0].value().to_string()
                }
                continue;
            }

            if child.tag_name().name() == "render" {
                let parsed = &self.parse_render_value(&child, root_u64_id);

                render_self.push_str(&javascript_assign("self", root_id));
                render_self.push_str(&parsed.0);
                continue;
            }

            if child.is_text() {
                let text = child.text().unwrap().trim();
                let value = preproc_attr_value(text);

                if !text.is_empty() {   
                    render_self.push_str(&format!(";{}.appendChild(document.createTextNode({}));", root_id, value));
                    render_self.push_str(&format!(";{}.appendChild({});", parent_id, root_id));
                }
            } else {
                let rendered_child = self.rendering_script_from_desc(child, root_id.to_string());
                render_self.push_str(&format!(";{};", rendered_child.rendering_script));
                render_self.push_str(&format!(";{}.appendChild({});", root_id, &cream_dom_name(rendered_child.root_dom_id)));
            }
        }

        rendering_script.push_str(&javascript_function(
            format!("render_fn{rand}"), &render_self, vec![]
        ));
        rendering_script.push_str(&format!(";{};", post_render_hooks));
        rendering_script.push_str(&javascript_function_call(&format!("render_fn{rand}"), vec![]));
        rendering_script.push(';');

        if is_comp {
            rendering_script.push_str(&javascript_assign(
                root_id, 
                &javascript_function_call(node.tag_name().name(), vec![cream_object(comp_attr_map_id)])
            ));
        }
        
        RenderReturn { 
            rendering_script, script, root_dom_id: root_u64_id, comp_name 
        }
    }
}
