use std::path::Path;
use std::path::PathBuf;
use oxc_diagnostics::OxcDiagnostic;
use roxmltree::Attribute;
use crate::consts::FRAGMENT;
use crate::helpers::javascript::javascript_function::javascript_function;
use crate::helpers::javascript::javascript_init_var::NULL;
use crate::helpers::javascript::javascript_init_var::javascript_init_var;
use crate::helpers::javascript::javascript_function_call::javascript_function_call;
use crate::helpers::javascript::javascript_assign::javascript_assign;
use crate::helpers::javascript::javascript_string::javascript_string;
use crate::std_err::ErrType;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;
use roxmltree::Node;
use roxmltree::Error;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Component {
    pub html: String,
    pub name: String,
    pub out: String,
}

pub fn special_trim(s: String) -> String {
    s
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn cream_dom_name(id: u64) -> String {format!("cream_element{id}")}
pub fn cream_component(id: u64) -> String {format!("cream_component{id}")}
pub fn cream_object(id: u64) -> String {format!("cream_object{id}")}
pub fn std_lib_path(name: &str) -> String {
    let mut path = PathBuf::from(name);
    path.set_extension("js");

    let out_path = Path::new("./.cream_std/")
        .join(path);

    out_path.to_string_lossy().into_owned()
}

#[derive(Debug, Clone)]
pub enum Directive {
    For(String),
    If(String),
    None
}

impl Directive {
    pub fn from(s: String, val: String) -> Self {
        match s.as_str() {
            "c-for" => Directive::For(val),
            "c-if" => Directive::If(val),
            _ => Directive::None
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Directive::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[derive(Default)]
pub struct RenderReturn {
    pub rendering_script: String,
    pub script: String,
    pub root_dom_id: u64,
    pub comp_name: String,
    pub dirs: Vec<Directive>
}

fn preproc_attr_value(s: &str) -> String {
    if let Some(val) = s.strip_prefix("@") {
        val.to_string()
    } else {
        format!("`{s}`")
    }
}

fn default_component_param() -> String {
    String::from("{childrens: []}")
}

pub fn format_oxc_diag(diag: &OxcDiagnostic, src: String) -> String {
    format!("{} at {}:{}", diag.message, src, diag.labels.clone()[0].offset())
}

fn is_special_attr_for_comp(name: &str) -> bool {
    let dir = Directive::from(name.to_string(), String::new());
    dir.is_some() || name == "subscribe"
}

fn wrap_directive(dirs: &Vec<Directive>, inside_script: String) -> String {
    let mut wrapped = inside_script;
    for dir in dirs.iter().rev() {
        match dir {
            Directive::For(stmt) => {
                wrapped = format!(";for({}){{{}}};", stmt, wrapped)
            }
            Directive::If(stmt) => {
                wrapped = format!(";if({}){{{}}};", stmt, wrapped)
            },
            Directive::None => {}
        }
    }

    wrapped
}

impl Component {
    pub fn new(
        html: String,
        name: String,
    ) -> Self {
        Self {
            html,
            name,
            out: String::new()
        }
    }

    fn parse_render_value(&self, node: &Node, id: u64) -> (String, Vec<u64>) {
        let mut ret = String::new();
        let mut ids = vec![];

        for child in node.children().rev() {
            if child.is_text() || child.is_comment() {
                ret.insert_str(0, child.text().unwrap());
                continue;
            }

            let code = self.rendering_script_from_desc(child, false);
            ret.insert_str(0, &javascript_function_call(
                &format!("{}.appendChild", cream_dom_name(id)),
                vec![
                    format!("{} ?? {FRAGMENT}", cream_dom_name(code.root_dom_id))
                ]
            ));
            ret.insert_str(0, &code.rendering_script);
            ids.push(code.root_dom_id);
        }

        (ret, ids)
    }

    pub fn html_rendering_script(&self) -> Result<RenderReturn, Error>  {
        let synt = format!("<div>{}</div>", &self.html.trim());
        let res = roxmltree::Document::parse(&synt)?;
        Ok(self.rendering_script_from_desc(res.root_element(), false))
    }

    fn subscribe_fn_name() -> String {"subscribe".to_string()}
    pub fn cream_window_obj() -> String {"window.__CREAM__".to_string()}
    fn element_storing_object() -> String {"window.__CREAM__.elements".to_string()}

    fn rendering_script_from_desc(&self, node: Node, is_parent_comp: bool) -> RenderReturn {
        if node.tag_name().name() == "append" {
            StdErr::exec(SyntaxError, "'append' tag is expected to have a parent which is not a 'render' tag");
        }
        
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
                &javascript_init_var(&cream_object(comp_attr_map_id), &default_component_param())
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
            String::new()
        } else {
            format!("del && {root_id}.replaceChildren();")
        };

        let mut post_render_hooks = String::new();
        let rand =  rng.next_u64();
        let mut dirs = vec![];
            
        for attr in node.attributes() {
            if is_comp && !is_special_attr_for_comp(attr.name()) {
                let value = &preproc_attr_value(attr.value());
                render_self.push_str(&javascript_assign(
                    &format!("{}[{:?}]", cream_object(comp_attr_map_id), attr.name()),
                    value
                ));
                continue;
            }

            let dir = Directive::from(attr.name().to_string(), attr.value().to_string());
            match dir {
                Directive::None => {},
                _ => {
                    dirs.push(dir);
                    continue;
                }
            }

            if attr.name().starts_with("on_") {
                let event = attr.name()[3..].to_string();
                render_self.push_str(&javascript_function_call(
                    &format!("{}.addEventListener", root_id), 
                    vec![javascript_string(&event), attr.value().to_string()]
                ));
            } else if attr.name() == "key" {
                let value = preproc_attr_value(attr.value());
                render_self.push_str(&format!(";{}[{}] = [{}, ...({}[{}] ?? [])];", 
                    Component::element_storing_object(), value, root_id, 
                    Component::element_storing_object(), value
                ));
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
                    &format!("{}[{:?}]", root_id, attr.name()),
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
            } else if child.tag_name().name() == "append" {
                let mut obj = String::new();
                for grandchild in child.children() {
                    if !grandchild.is_text() {
                        StdErr::exec(SyntaxError, "Childrens of 'append' were not expected")
                    } else {
                        obj.push_str(grandchild.text().unwrap())
                    }
                }
                render_self.push_str(&format!(";{}.appendChild({});", root_id, obj));
                continue;
            }


            if child.is_text() {
                let text = child.text().unwrap().trim();
                let value = preproc_attr_value(text);

                if !text.is_empty() && !is_parent_comp {
                    render_self.push_str(&format!(";{}.appendChild(document.createTextNode({}));", root_id, value));
                } else if !text.is_empty() {
                    render_self.push_str(&format!(";{}.appendChild(document.createTextNode({}));", root_id, value));
                }
            } else if !is_comp {
                let rendered_child = self.rendering_script_from_desc(child, false);
                render_self.push_str(&format!(";{};", rendered_child.rendering_script));
                render_self.push_str(&format!(";{}.appendChild({});", root_id, 
                    format!("{} ?? {FRAGMENT}", &cream_dom_name(rendered_child.root_dom_id))
                ));
                render_self = wrap_directive(&rendered_child.dirs, render_self);
            } else {
                let rendered_child = self.rendering_script_from_desc(child, true);
                render_self.push_str(&format!(";{};", rendered_child.rendering_script));
                render_self.push_str(&format!(";{}.childrens.push({});", cream_object(comp_attr_map_id), &cream_dom_name(rendered_child.root_dom_id)));
                render_self = wrap_directive(&rendered_child.dirs, render_self);
            }
        }


        if is_comp {
            render_self.push_str(&javascript_assign(
                root_id, 
                &javascript_function_call(node.tag_name().name(), vec![cream_object(comp_attr_map_id)])
            ));
        }
        
        rendering_script.push_str(&javascript_function(
            format!("render_fn{rand}"), &render_self, vec!["del=true".to_string()]
        ));
        rendering_script.push_str(&format!(";{};", post_render_hooks));
        rendering_script.push_str(&javascript_function_call(&format!("render_fn{rand}"), vec!["false".to_string()]));
        rendering_script.push(';');
        
        RenderReturn { 
            rendering_script,
            script,
            root_dom_id: root_u64_id,
            comp_name,
            dirs
        }
    }
}
