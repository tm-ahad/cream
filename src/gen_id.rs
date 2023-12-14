use crate::helpers::javascript_function_call::javascript_function_call;
use crate::helpers::javascript_string::javascript_string;
use crate::component_markup::ComponentMarkUp;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::id_gen::IdGen;
use crate::import_base::ImportBase;
use crate::import_lib::add_lib;

pub fn gen_id(
    script: &mut String,
    dyn_script: &mut String,
    html: &mut ComponentMarkUp,
    import_base: &mut ImportBase,
    comp: bool,
    lang: &str,
    f_name: &str,
) {
    while let Some(id) = html.stat.find("@gen") {
        let id_p_8 = id + 5;
        let mut gen_id = String::new();
        let mut name = String::new();

        let mut named = false;

        let rep = if &html.stat[id + 4..id_p_8] == ":" {
            let end = read_until(&html.stat, id_p_8, ";", f_name, ComponentPart::Template);

            gen_id = IdGen::gen_string();
            let lib = &format!("id_gen.{lang}");

            add_lib(script, import_base, lib);
            name = html.stat[id_p_8..end].to_string();

            script.insert_str(0, &format!("var {}={}\n", name, javascript_string(&gen_id)));
            println!("{}", script);

            if comp {
                dyn_script.insert_str(0, &format!("var {}=gen_id()\n", name));
            }

            named = true;
            end
        } else {
            id + 4
        };

        html.stat.replace_range(id..rep+1, &javascript_string(&gen_id));

        if named {
            html.dynamic.replace_range(id..rep+1, &name);
        } else {
            gen_id = IdGen::gen_string();
            let lib = &format!("id_gen.{lang}");

            add_lib(script, import_base, lib);
            html.dynamic.replace_range(id..rep+1, &javascript_function_call("gen_id", vec![]));
        }
    }
}
