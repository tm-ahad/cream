use crate::id_gen::IdGen;

pub fn _gen_id(js: &mut String, html: &mut String) {
    while let Some(id) = html.find("@gen_id") {
        let id_p_8 = id + 8;
        let mut gen_id = String::new();

        let rep = if &html[id + 7..id_p_8] == ":" {
            let mut end = id_p_8;

            while &html[end..end + 1] != " "
                && &html[end..end + 1] != ">"
                && &html[end..end + 1] != "/"
            {
                end += 1;
            }

            gen_id = IdGen::get_and_update();

            js.insert_str(0, &format!("var {}=\"{gen_id}\"\n", &html[id_p_8..end]));

            end
        } else {
            id + 7
        };

        html.replace_range(id..rep, &format!("\"{}\"", &gen_id));
    }
}
