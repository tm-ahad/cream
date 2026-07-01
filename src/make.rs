use crate::consts::{BUILD_ENTRY, BUILD_PATH, ENTRY_FILE, HEAD_PREFIX_PATH};
use crate::helpers::dependancy_graph::DependencyGraph;
use crate::std_err::ErrType::NotFound;
use crate::std_err::{ErrType, StdErr};
use crate::{component::Component, config::Config};
use crate::helpers::build_source::{build_path, translate_import_src_to_build};
use std::process::exit;
use crate::out::out;
use std::fs::{self, read_to_string};

pub fn make(conf: &Config) {
    for str_val in conf.build.iter() {
        let mut dep_graph = DependencyGraph::new();
        let mut comp = Component::new(String::new(), str_val.to_string());
        comp.transpile(&conf, &mut dep_graph);
        out(&build_path(str_val), comp.out);
        dep_graph.install(BUILD_PATH);
    }

    let head_prefix = read_to_string(HEAD_PREFIX_PATH)
        .unwrap_or_else(|e| {
            StdErr::exec(NotFound, &format!(
                "failed to read {}: {}",
                HEAD_PREFIX_PATH, e
            ));
            exit(1)
        });
    

    let final_html = format!(
        "{head_prefix}
        <script type=\"module\">
            import {{mount as App}} from \"{}\"
            App({{}})
        </script>",
        translate_import_src_to_build(ENTRY_FILE, &conf.root)
    );

    fs::write(BUILD_ENTRY, final_html)
        .unwrap_or_else(|e| {
            StdErr::exec(ErrType::OSError, &format!(
                "Having trouble writing {}: {}",
                build_path(ENTRY_FILE), e
            ));
            exit(1);
        })

}
