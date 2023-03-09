use std::fs;
use std::fs::read_to_string;
use crate::collect_gen::collect_gen;
use crate::component::component;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::component::Component;

pub fn compile(name: &String, mut state: _StateBase) {
    let app = read_to_string(format!("./{}/src/app.js", name))
        .expect("app.js not found");
    let mut imports: Vec<Component> = vec![];
    let mut names: Vec<String> = vec![];

    let main_app = collect_gen(app.clone(), "app {".to_string(),
        0, "}");

    let mut js = String::new();
    let split = main_app.split("\n");

    match app.find("import component") {
        None => {}
        Some(e) => {
            let mut namei = e+17;
            let mut ci = e+30;
            let mut cn: String = String::new();
            let mut fnm: String = String::new();

            while &app[namei..namei+4] != "from" {
                cn.push(app.chars().nth(namei).unwrap());
                namei += 1;
            }

            while &app[ci..ci+1] != "\n" {
                fnm.push(app.chars().nth(ci).unwrap());
                ci+=1
            }

            names.push(app[e+16..namei].trim().to_string());
            imports.push(component(name, fnm.to_string(), cn.trim().to_string()));
        }
    }

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else { break }
    }

    let mut html = collect_gen(main_app, "<html>".to_string(),
                               0, "</html>");

    js = _state(js.clone(), &mut state);

    js = js.replace(".single()", "");

    #[allow(unused_assignments)]
    let mut fail: String = String::new();

    for n in names {

        fail = format!("<{}/>", n);
        let m = fail.as_str();

        println!("{fail}");

        if html.contains(m.clone()) {
            for i in &imports {
                if i.name == n {
                    js = format!("{}\n{}", js, i.js);
                    html = format!("{}\n{}", js, i.html)
                }
            }
        }
    }
    fs::write(format!("./{}/build/index.html", name), format!("
<body>
   {}
</body>
<script>
   {}
</script>
", html, js))
        .expect("File not found or writing not supported");

}