use crate::component::Component;
use crate::consts::{NEW_LINE, NIL, SBF, SBS, SPACE, UNTIL_TOKEN};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::find_component::find_component_by_name;
use crate::helpers::read_until::read_until;
use crate::std_err::ErrType::SyntaxError;
use crate::id_gen::IdGen;
use crate::javascript_lib::private_work_lib;
use crate::std_err::StdErr;
use crate::pass::pass;

#[allow(non_snake_case)]
pub fn UDT(
    comp_html: &mut String,
    script: &mut String,
    imports: &Vec<Component>,
    f_name: &str
) {
    let first = true;
    let ch_len = comp_html.len();

    while let Some(e) = comp_html.find(UNTIL_TOKEN) {
        let mut fall = e;

        while &comp_html[fall..fall + 1] != NEW_LINE && fall > 0 {
            fall -= 1;
        }

        let mut up = e + 7;
        let len = read_until(comp_html, ch_len, ">", f_name, ComponentPart::Template);

        while &comp_html[up..up + 1] != ">" && up < len {
            up += 1;
        }

        let mut do_ = NIL;
        let mut th = NIL;

        let bind = comp_html[fall..up].to_string();
        let li = bind.as_str();
        let li_len = li.len();

        match li.find("that=") {
            None => return,
            Some(e) => {

                let mut init = e + 5;

                while &li[init..init + 1] != SPACE
                    && &li[init..init + 1] != "/"
                    && &li[init..init + 1] != NEW_LINE
                {
                    if init == li_len-2 {
                        StdErr::exec(SyntaxError, &format!("'{SPACE}' or '/' or '\\n' expected in template ({f_name})"))
                    }
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
                    if init == li_len-2 {
                        StdErr::exec(SyntaxError, &format!("'{SPACE}' or '/' or '\\n' expected in template ({f_name})"))
                    }
                    init += 1
                }

                do_ = &li[e + 3..init]
            }
        }

        

        let do_comp = find_component_by_name(imports, do_.to_string())
            .unwrap_or_else(|| panic!("Couldn't find component {}", th));
        let id = IdGen::gen_string();

        comp_html.replace_range(
            fall..up,
            &format!("<div id={}>{}</div>", id, do_comp.html.stat),
        );

        let th_comp = find_component_by_name(imports, th.to_string())
            .unwrap_or_else(|| panic!("Couldn't find component {}", th));

        if first {
            script.push_str(&private_work_lib())
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
            th_comp.script, th_comp.html.stat
        ));
    }
}
