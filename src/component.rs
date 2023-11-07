use crate::collect_scope::collect_scope;
use crate::component_markup::ComponentMarkUp;
use crate::extract_component::extract_component;
use crate::helpers::expected::expect_some;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::import_script::import_script;
use crate::matcher::Matcher;
use crate::scope::{parse_scope, scopify};
use crate::script_module::module;
use crate::state_base::_StateBase;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::template::template;
use crate::transpile_component::transpile_component;
use crate::import_component::import_component;
use crate::transpile_to_js::transpile_script;
use crate::component_args::ComponentArgs;
use crate::consts::{DOUBLE_QUOTE, IGNORE_STATE , NEW_LINE_CHAR, NIL};
use crate::at_temp::at_temp;
use crate::comment::comment;
use crate::dsp_map::DspMap;
use crate::gen_id::gen_id;
use crate::router::router;
use crate::state::_state;
use crate::udt::UDT;
use rusty_v8::{self as v8, Script};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use crate::helpers::to_raw_parsable_format::to_raw_parsable_format;

pub struct Component {
    pub html: ComponentMarkUp,
    pub dyn_script: String,
    pub dom_script: String,
    pub script: String,
    pub name: String,
}

impl Component {
    pub fn new(
        dom_script: String,
        script: String,
        dyn_script: String,
        html: ComponentMarkUp,
        name: String
    ) -> Self {
        Component {
            dom_script,
            script,
            dyn_script,
            html,
            name
        }
    }

    pub fn static_transpiled(&self) -> String {
        let mu = &self.html.stat;
        let script = &self.script;

        format!(
            "\
            {mu}
            <script>
                {script}
            </script>
        "
        )
    }
}

impl Clone for Component {
    fn clone(&self) -> Self {
        Self {
            dom_script: self.dom_script.clone(),
            name: self.name.clone(),
            script: self.script.clone(),
            html: self.html.clone(),
            dyn_script: self.dyn_script.clone()
        }
    }
}

pub fn component(
    f_name: &str,
    c_name: &str,
    transpile_command: &str,
    config: &DspMap,
) -> Component {
    let isolate = &mut v8::Isolate::new(Default::default());

    let mut binding = v8::HandleScope::new(isolate);
    let scope = &mut binding;
    let context = v8::Context::new(scope);

    let mut binding = v8::ContextScope::new(scope, context);
    let scope = &mut binding;

    let import_base = &mut ImportBase::new();
    let st = &mut _StateBase::new();

    let component_args = ComponentArgs::new(transpile_command, config);

    let __script__ = &String::from("script");
    let lang = config.get("lang").unwrap_or(__script__);
    let path = format!("./{f_name}").replace(DOUBLE_QUOTE, NIL);

    let mut app = read_to_string(path).unwrap_or_else(|e| {
        StdErr::exec(OSError, &e.to_string());
        todo!()
    });

    app = app
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    comment(&mut app);

    let macher = Matcher::Component(c_name);
    let pat = expect_some(collect_scope(&app, &macher, false), &c_name);
    let main_app = pat.mp_val();
    let mut dom_script = String::new();

    let binding = &main_app;
    let split = binding.split('\n');

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

    let template_mp = expect_some(
        collect_scope(&main_app, &Matcher::Template, false),
        "Template",
    );

    let mut html = template_mp.mp_val();

    let mut cmu = ComponentMarkUp::new(html.clone(), html.clone());
    let imports = import_component(&mut app, &component_args);
    let mut ccm = BTreeMap::new();
    let mut scopes = Vec::new();
    let mut dyn_script = script.clone();

    gen_id(
        &mut script,
        &mut dyn_script,
        &mut cmu,
        import_base,
        true,
        lang,
    );

    extract_component(&mut ccm, &imports, &mut html);
    router(
        &mut cmu,
        &mut script,
        &component_args
    );

    import_lib(&mut app, import_base, &mut script);
    module(&mut app, import_base, &mut script);
    import_script(&mut app, import_base, &mut script);
    parse_scope(&mut script, &mut scopes);

    transpile_script(lang, transpile_command, &mut script);

    let string = v8::String::new(scope, &script).unwrap();
    let scr = Script::compile(scope, string, None).unwrap();
    let _ = scr.run(scope);

    script = script.replace(IGNORE_STATE, NIL).replace(".cam()", "");

    UDT(&mut html, &mut script, &imports);
    import_npm(&mut app, &mut script);
    scopify(&mut script, scopes, config, st);

    template(&mut cmu, &mut dom_script, scope, st);
    _state(&mut script, st);

    let script_writer_ptr = &mut script;
    let html_writer_ptr = &mut html;

    transpile_component(
        ccm,
        script_writer_ptr,
        html_writer_ptr,
        to_raw_parsable_format(
            &*script_writer_ptr,
            &*html_writer_ptr
        )
    );

    at_temp(&mut html, &mut dom_script, st, scope);

    Component::new(
        dom_script,
        script,
        dyn_script,
        cmu,
        c_name.to_string(),
    )
}

pub fn component_call(id: u32) -> String {
    format!("//COMPONENT CALL: {id}")
}

pub fn component_call_len(dnl: u8) -> usize {
    (18 + dnl) as usize
}
