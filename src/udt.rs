use crate::component::Component;
use crate::consts::{NEW_LINE, NIL, SBF, SBS, SPACE, UNTIL_TOKEN};
use crate::helpers::find_component::find_component_by_name;
use crate::id_gen::IdGen;
use crate::pass::pass;

#[allow(non_snake_case)]
pub fn UDT<'a>(comp_html: &mut String, script: &mut String, imports: &Vec<Component>) {
    let first = true;

    while let Some(e) = comp_html.find(UNTIL_TOKEN) {
        let mut fall = e;

        while &comp_html[fall..fall + 1] != NEW_LINE && fall > 0 {
            fall -= 1;
        }

        let mut up = e + 7;
        let len = comp_html.len();

        while &comp_html[up..up + 1] != ">" && up < len {
            up += 1;
        }

        let mut do_ = NIL;
        let mut th = NIL;

        let bind = comp_html[fall..up].to_string();
        let li = bind.as_str();

        match li.find("that=") {
            None => {return ()},
            Some(e) => {
                let mut init = e + 5;

                while &li[init..init + 1] != SPACE
                    && &li[init..init + 1] != "/"
                    && &li[init..init + 1] != NEW_LINE
                {
                    init += 1
                }

                th = &li[e + 5..init]
            }
        }

        match li.find("do=") {
            None => pass(),
            Some(e) => {
                let mut init = e + 3;

                while &li[init..init + 1] != SPACE
                    && &li[init..init + 1] != "/"
                    && &li[init..init + 1] != NEW_LINE
                {
                    init += 1
                }

                do_ = &li[e + 3..init]
            }
        }

        let id;

        {
            let do_comp = find_component_by_name(imports, do_.to_string())
                .unwrap_or_else(|| panic!("Couldn't find component {}", th));
            id = IdGen::gen_string();

            comp_html.replace_range(
                fall..up,
                &format!("<div id={}>{}</div>", id, do_comp.html.stat),
            );
        }

        let th_comp = find_component_by_name(imports, th.to_string())
            .unwrap_or_else(|| panic!("Couldn't find component {}", th));

        if first {
            script.push_str(
                "
class Work {
    #value;
    constructor(init) {
        this.#value = init;
    }

    do(then) {
        try {
            let _res = this.#value();

            let res = then({
                state: \"done\",
                error: null,
                value: _res
            });

            return res;
        } catch (e) {
           throw e;
        }
    }
}\n",
            )
        }
        script.push_str(&format!(
            "\
let work = new Work(function() {SBF}
    {}
{SBS})

work.do(function() {SBF}
    let ptr = document.getElementById(\"{id}\")
    ptr.innerHTML = `{}`
{SBS})
        ",
            th_comp.dom_script, th_comp.html.stat
        ));
    }
}
