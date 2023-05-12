use crate::id_gen::IdGen;

pub fn _gen_id(js: &mut String, html: &mut String) {
    while let Some(id) = html.find("@gen_id") {
        let id_p_8 = id+8;

        let rep = if &html[id+7..id_p_8] == ":" {
            let gen_id = IdGen::get_and_update();
            let mut end = id_p_8;

            while &html[end..end+1] != " " &&  
                  &html[end..end+1] != "<" &&
                  &html[end..end+1] != ">" {
                  end += 1;
            }

            js.insert_str(0, &format!("var {}=\"{gen_id}\"\n", &html[id_p_8..end]));

            end
        } else {id+7};

        let gen_id = IdGen::get_and_update();
        html.replace_range(id..rep, &format!("\"{}\"", &gen_id));
    }
}
