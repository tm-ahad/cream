use crate::component_markup::ComponentMarkUp;
use crate::helpers::interpolate_string::interpolate_string;
use crate::helpers::javascript_string::javascript_string;
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
) {
    while let Some(id) = html.stat.find("@gen") {
        let id_p_8 = id + 8;
        let mut gen_id = String::new();

        let rep = if &html.stat[id + 7..id_p_8] == ":" {
            let mut end = id_p_8;

            while &html.stat[end..end + 1] != " "
                && &html.stat[end..end + 1] != ">"
                && &html.stat[end..end + 1] != "/"
            {
                end += 1;
            }

            gen_id = IdGen::gen_string();
            let lib = match lang {
                "js" => "id_gen",
                "ts" => "id_gen_ts",
                _ => todo!(),
            };

            add_lib(script, import_base, lib);
            let name = &html.stat[id_p_8..end];

            script.insert_str(0, &format!("var {}={}\n", name, javascript_string(&gen_id)));

            if comp {
                dyn_script.insert_str(0, &format!("var {}=gen_id()\n", name));
            }

            end
        } else {
            id + 7
        };

        html.stat
            .replace_range(id..rep, &javascript_string(&gen_id));
        html.dynamic
            .replace_range(id..rep, &interpolate_string(&gen_id));
    }
}
