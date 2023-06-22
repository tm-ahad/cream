use crate::component::Component;
use crate::id_gen::IdGen;

#[allow(non_snake_case)]
pub fn UDT(comp_html: &mut String, js: &mut String, imports: &Vec<Component>) {
    let first = true;

    while let Some(e) = comp_html.find("<Until ") {
        let mut fall = e;

        while &comp_html[fall..fall+1] != "\n" && fall > 0 {
            fall -= 1;
        }

        let mut up = e + 7;
        let len = comp_html.len();

        while &comp_html[up..up+1] != ">" && up < len {
            up += 1;
        }

        let li = &comp_html[fall..up+1];
        let mut th = String::new();
        let mut do_ = String::new();

        match li.find("that=") {
            None => {}
            Some(e) => {
                let mut init = e + 5;

                while &li[init..init+1] != " " &&
                    &li[init..init+1] != "/" &&
                    &li[init..init+1] != "\n" {
                    init += 1
                }

                th = String::from(&li[e+5..init])
            }
        }

        match li.find("do=") {
            None => {}
            Some(e) => {
                let mut init = e + 3;

                while &li[init..init+1] != " " &&
                    &li[init..init+1] != "/" &&
                    &li[init..init+1] != "\n" {
                    init += 1
                }

                do_ = String::from(&li[e+3..init])
            }
        }

        let mut th_comp = &Component::NEW;
        let mut do_comp = &Component::NEW;

        for i in imports {
            if i.name == th {
                th_comp = i
            }
        }

        for i in imports {
            if i.name == do_ {
                do_comp = i
            }
        }

        let id = IdGen::get_and_update();

        let cb1 = "{";
        let cb2 = "}";

        comp_html.replace_range(fall..up, &format!("<div id={}>{}</div>", id , do_comp.html));

        if first {
            js.push_str("
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
}\n")
        }
        js.push_str(&format!("\
let work = new Work(function() {cb1}
    {}
{cb2})

work.do(function() {cb1}
    let ptr = document.getElementById(\"{id}\")
    ptr.innerHTML = `{}`
{cb2})
        ", th_comp.js, th_comp.html));
    }
}