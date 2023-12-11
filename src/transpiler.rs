use crate::consts::{CAM, DEFAULT_COMPILATION_PATH, IGNORE_STATE, NEW_LINE_CHAR, NIL};
use crate::at_temp::at_temp;
use crate::collect_scope::collect_scope;
use crate::comment::comment;
use crate::component_args::ComponentArgs;
use crate::component_markup::ComponentMarkUp;
use crate::dsp_map::DspMap;
use crate::extract_component::extract_component;
use crate::gen_id::gen_id;
use crate::helpers::merge_dom_script::merge_dom_script;
use crate::transpile_component::transpile_component;
use crate::transpile_to_js::transpile_script;
use crate::import_component::import_component;
use crate::import_template::import_template;
use crate::helpers::expected::expect_some;
use crate::import_script::import_script;
use crate::scope::{parse_scope, scopify};
use crate::import_html::import_html;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::script_module::module;
use crate::state_base::_StateBase;
use crate::import_ext::import_ext;
use crate::template::template;
use crate::matcher::Matcher;
use crate::remove::remove;
use crate::router::router;
use crate::state::_state;
use crate::udt::UDT;
use crate::out::out;
use std::collections::BTreeMap;
use std::fs::read_to_string;

pub fn transpile(mut state: _StateBase, mut import_base: ImportBase, config: &DspMap) {
    let binding = String::from("script");
    let lang = config.get("lang").unwrap_or(&binding);

    let binding = String::new();
    let transpile_command = config.get("build").unwrap_or(&binding);

    let src = &format!("./src/app.{lang}");
    let mut app = read_to_string(src).expect("Project or app.nts not found");
    let mut dom_script = String::new();

    app = app
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    comment(&mut app);

    let mut ccm = BTreeMap::new();
    let binding = String::from("app");
    let app_matcher = Matcher::Component(&binding);

    let pat = expect_some(collect_scope(&app, &app_matcher, false), "App component");
    let main_app = pat.mp_val();

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

    remove(&mut script, src);
    import_script(&mut app, &mut import_base, &mut script, src);
    import_template(&mut app, src, &mut html);

    let mut cmu = ComponentMarkUp::new(html.clone(), html.clone());

    let mut scopes: Vec<String> = Vec::new();
    gen_id(
        &mut script,
        &mut String::new(),
        &mut cmu,
        &mut import_base,
        false,
        lang,
        src
    );

    import_lib(&mut app, &mut import_base, &mut script, src);
    transpile_script(lang, transpile_command, &mut script);
    parse_scope(&mut script, &mut scopes);

    let component_args = ComponentArgs::new(transpile_command, config);
    let imports = import_component(&mut app, &component_args, src);

    extract_component(&mut ccm, &imports, &mut cmu, src);
    module(&mut app, &mut import_base, &mut script, src);

    script = script
        .replace(IGNORE_STATE, NIL)
        .replace(CAM, NIL);

    UDT(&mut html, &mut script, &imports, src);
    import_npm(&mut app, &mut script, src);

    template(&mut cmu, &mut dom_script, &mut state, src);
    scopify(&mut script, scopes, config, &mut state, src);

    let script_writer_ptr = &mut dom_script;
    at_temp(&mut cmu, script_writer_ptr, src);

    transpile_component(
        ccm,
        script_writer_ptr,
        &mut cmu,
    );

    merge_dom_script(&mut script, &dom_script);
    _state(&mut script, &mut state, src);
    import_ext(&mut app, src, &mut script);
    import_html(&mut app, src, &mut html);

    script.insert_str(0, &router(config));

    let binding = String::from(DEFAULT_COMPILATION_PATH);
    let _app_html = config.get("_app_html").unwrap_or(&binding);

    out(_app_html, cmu.stat, script, config);
}
