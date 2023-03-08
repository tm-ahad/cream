use std::fs;
use std::fs::read_to_string;
use crate::collect_gen::collect_gen;
use crate::state::_state;
use crate::state_base::_StateBase;

pub fn compile(name: &String, mut state: _StateBase) {
    let app = read_to_string(format!("./{}/src/app.js", name))
        .expect("app.js not found");

    let main_app = collect_gen(app.clone(), "app {".to_string(),
        0, "}");

    let mut js = String::new();
    let split = main_app.split("\n");

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else { break }
    }

    let html = collect_gen(main_app, "<html>".to_string(),
                           0, "<html/>");

    js = _state(js.clone(), &mut state);

    js = js.replace(".single()", "");
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