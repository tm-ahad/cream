use crate::at_temp::at_temp;
use crate::collect_scope::collect_scope;
use crate::comment::comment;
use crate::component_args::ComponentArgs;
use crate::component_markup::ComponentMarkUp;
use crate::consts::{CAM, DEFAULT_COMPILATION_PATH, IGNORE_STATE, NIL};
use crate::dsp_map::DspMap;
use crate::extract_component::extract_component;
use crate::gen_id::gen_id;
use crate::helpers::expected::expect_some;
use crate::helpers::merge_dom_script::merge_dom_script;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::import_script::import_script;
use crate::matcher::Matcher;
use crate::out::out;
use crate::remove::remove;
use crate::router::router;
use crate::scope::{parse_scope, scopify};
use crate::script_module::module;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::template::template;
use crate::transpile_component::transpile_component;
use crate::transpile_to_js::transpile_script;
use crate::udt::UDT;
use rusty_v8::{self as v8, Script};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use crate::helpers::to_raw_parsable_format::to_raw_parsable_format;
use crate::import_component::import_component;

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
            script.push('\n');
            script.push_str(s)
        } else {
            break;
        }
    }

    remove(&mut script);

    let mut comp_html = expect_some(
        collect_scope(&main_app, &Matcher::Template, false),
        "Template",
    )
    .mp_val();

    let mut cmu = ComponentMarkUp::new(comp_html.clone(), comp_html.clone());

    let mut scopes: Vec<String> = Vec::new();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    gen_id(
        &mut script,
        &mut String::new(),
        &mut cmu,
        &mut import_base,
        false,
        lang,
    );

    import_lib(&mut app, &mut import_base, &mut script);
    parse_scope(&mut script, &mut scopes);

    transpile_script(lang, transpile_command, &mut script);

    let isolate = &mut v8::Isolate::new(Default::default());

    let mut binding = v8::HandleScope::new(isolate);
    let scope = &mut binding;
    let context = v8::Context::new(scope);

    let mut binding = v8::ContextScope::new(scope, context);
    let scope = &mut binding;

    let component_args = ComponentArgs::new(transpile_command, config);
    let imports = import_component(&mut app, &component_args);

    extract_component(&mut ccm, &imports, &mut script, &mut comp_html);
    router(
        &mut cmu,
        &mut script,
        &component_args,
    );

    import_script(&mut app, &mut import_base, &mut script);
    module(&mut app, &mut import_base, &mut script);

    let ben = &script.replace(CAM, NIL);
    println!("{}", ben);
    let code = v8::String::new(scope, ben).unwrap();

    let mut _script = Script::compile(scope, code, None).unwrap();
    let _ = _script.run(scope).unwrap();

    script = script
        .replace(IGNORE_STATE, NIL)
        .replace(CAM, NIL);

    UDT(&mut comp_html, &mut script, &imports);
    import_npm(&mut app, &mut script);

    at_temp(&mut comp_html, &mut dom_script, &mut state, scope);
    template(&mut cmu, &mut dom_script, scope, &mut state);
    _state(&mut script, &mut state);
    scopify(&mut script, scopes, config, &mut state);

    let script_writer_ptr = &mut script;
    let html_writer_ptr = &mut comp_html;

    transpile_component(
        ccm,
        script_writer_ptr,
        html_writer_ptr,
        to_raw_parsable_format(
            &*script_writer_ptr,
            &*html_writer_ptr
        )
    );

    let binding = String::from(DEFAULT_COMPILATION_PATH);
    let _app_html = config.get("_app_html").unwrap_or(&binding);
    merge_dom_script(&mut script, &dom_script);

    out(_app_html, comp_html, script, config);
}
