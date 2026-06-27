use crate::consts::{BUILD_ENTRY, BUILD_PATH, ENTRY_FILE};
use crate::helpers::dependancy_graph::DependancyGraph;
use crate::std_err::{ErrType, StdErr};
use crate::{component::Component, config::Config};
use crate::helpers::build_source::{build_path, translate_import_src_to_build};
use std::process::exit;
use std::{fs, io::Error, path::Path};
use crate::out::out;

fn clear_dir(path: &Path) -> Result<(), Error> {
    if !path.is_dir() {
        return Ok(())
    }

    for child in fs::read_dir(path)? {
        let path = child?.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

pub fn router(conf: &mut Config) {
    let _ = clear_dir(Path::new(BUILD_PATH));
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
