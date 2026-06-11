use crate::component::Component;
use crate::consts::{DEFAULT_COMPILATION_PATH, NEW_LINE_CHAR};
use crate::helpers::write_component::write_component;
use crate::import_component::import_component;
use crate::import_template::import_template;
use crate::helpers::expected::expect_some;
use crate::collect_scope::collect_scope;
use crate::component_args::ComponentArgs;
use crate::import_script::import_script;
use crate::import_html::import_html;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::script_module::module;
use crate::dsp_map::DspMap;

use crate::import_ext::import_ext;
use crate::matcher::Matcher;
use std::fs::read_to_string;
use crate::component_map::ComponentMap;

pub fn transpile_component_(
    import_base: &mut ImportBase, 
    config: &DspMap,
    f_name: String,
    c_name: String
) -> Component {
    let mut app = read_to_string(f_name.clone()).expect(&*format!("{f_name} not found"));

    let app_trimmed = app
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    let binding = c_name.clone();
    let app_matcher = Matcher::Component(&binding);
    let pat = expect_some(collect_scope(&app_trimmed, &app_matcher, false), &*format!("{c_name} component"));
    let main_app = pat.mp_val();

    let mut component_map = ComponentMap::new(ComponentArgs::new(config.clone(), import_base.clone()));
    let split = main_app.split('\n'); 

    let mut script = String::new();
    let binding = Matcher::Template.to_string();
    let t = binding.as_str();

    for s in split {
        if s != t {
            script.push(NEW_LINE_CHAR);
            script.push_str(s)
        } else {
            break;
        }
    }

    let mut html = expect_some(
        collect_scope(&main_app, &Matcher::Template, false),
        "Template",
    )
        .mp_val();

    import_script(&mut app, import_base, &mut script, &f_name);
    import_template(&mut app, &f_name, &mut html);

    import_lib(&mut app, import_base, &mut script, &f_name);
    module(&mut app, import_base, &mut script, &f_name);
    import_npm(&mut app, &mut script, &f_name);
    import_ext(&mut app, &f_name, &mut script);
    import_html(&mut app, &f_name, &mut html);

    let binding = String::from(DEFAULT_COMPILATION_PATH);
    let _app_html = config.get("_app_html").unwrap_or(&binding);

    import_base.patch(&mut script);
    {
        let imports = import_component(&app, f_name.to_string(), &mut component_map);
        for comp in imports {
            script.insert_str(0, &write_component(comp));
        }
    }
    Component::new(script, html, c_name, String::new())
}

