use crate::consts::{BUILD_ENTRY, BUILD_PATH, ENTRY_FILE};
use crate::helpers::dependancy_graph::DependancyGraph;
use crate::std_err::{ErrType, StdErr};
use crate::{component::Component, config::Config};
use crate::helpers::build_source::{build_path, translate_import_src_to_build};
use std::process::exit;
use crate::out::out;
use std::fs;

pub fn make(conf: &Config) {
    for str_val in conf.build.iter() {
        let mut dep_graph = DependancyGraph::new();
        let mut comp = Component::new(String::new(), str_val.to_string(), &mut dep_graph);
        comp.transpile(&conf);
        out(&build_path(str_val), comp.out);
        dep_graph.install(BUILD_PATH);
    }
    

    let final_html = format!("
    <script type=\"module\">
        import {{mount as App}} from \"{}\"
        App({{}})
    </script>
    ", translate_import_src_to_build(ENTRY_FILE));

    fs::write(BUILD_ENTRY, final_html)
        .unwrap_or_else(|e| {
            StdErr::exec(ErrType::OSError, &format!(
                "Having trouble writing {}: {}",
                build_path(ENTRY_FILE), e
            ));
            exit(1);
        })

}
