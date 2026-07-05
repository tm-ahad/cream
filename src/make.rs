use crate::consts::{
    BUILD_ENTRY, BUILD_PATH, CHECK_SUM_FILE, CONFIG_FILE, ENTRY_FILE, HEAD_PREFIX_PATH,
};
use crate::helpers::build_source::{build_path, translate_import_src_to_build};
use crate::helpers::check_sums::CheckSums;
use crate::helpers::dependancy_graph::DependencyGraph;
use crate::out::out;
use crate::std_err::ErrType::{NotFound, OSError};
use crate::std_err::{ErrType, StdErr};
use crate::{component::Component, config::Config};
use notify::Event;
use std::collections::BTreeMap;
use std::fs::{self, read_to_string};
use std::path::PathBuf;
use std::process::exit;

pub fn make(conf: &Config, change: &Option<Event>, check_sums: &mut CheckSums) {
    let mut new_check_sum = BTreeMap::new();

    for str_val in conf.build.iter() {
        let mut dep_graph = DependencyGraph::new();
        let mut comp = Component::new(None, str_val.to_string());
        let path_buf = fs::canonicalize(PathBuf::from(str_val)).unwrap_or_else(|e| {
            StdErr::exec(OSError, &format!("Error canonicalize {str_val}: {e}"));
            exit(1)
        });

        let cannon_config = fs::canonicalize(PathBuf::from(CONFIG_FILE)).unwrap_or_else(|e| {
            StdErr::exec(OSError, &format!("Error canonicalize {str_val}: {e}"));
            exit(1)
        });

        let cannon_path = fs::canonicalize(PathBuf::from(str_val))
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let hash = check_sums.get(&cannon_path);
        if let Some(event) = change {
            if event.paths.contains(&path_buf) || event.paths.contains(&cannon_config) {
                comp.transpile(&conf, &mut dep_graph);
                out(&build_path(str_val), comp.output());
                dep_graph.install(BUILD_PATH, conf);
            }
        } else if change.is_none() && hash != Some(&comp.hash()) {
            println!("Making {str_val}...");
            comp.transpile(&conf, &mut dep_graph);
            out(&build_path(str_val), comp.output());
            dep_graph.install(BUILD_PATH, conf);
        } else {
            println!("Skipping {str_val} (previous checksum matched)");
        }

        new_check_sum.insert(cannon_path, comp.hash());
    }

    let head_prefix = read_to_string(HEAD_PREFIX_PATH).unwrap_or_else(|e| {
        StdErr::exec(
            NotFound,
            &format!("failed to read {}: {}", HEAD_PREFIX_PATH, e),
        );
        exit(1)
    });

    let final_html = format!(
        "{head_prefix}\n<script type=\"module\">import {{mount}} from \"{}\";mount({{}})</script>",
        translate_import_src_to_build(ENTRY_FILE, &conf.root)
    );

    fs::write(
        CHECK_SUM_FILE,
        serde_json::to_string::<CheckSums>(&new_check_sum).unwrap(),
    )
    .unwrap_or_else(|e| {
        StdErr::exec(
            ErrType::OSError,
            &format!("Having trouble writing {}: {}", build_path(ENTRY_FILE), e),
        );
        exit(1);
    });

    fs::write(BUILD_ENTRY, final_html).unwrap_or_else(|e| {
        StdErr::exec(
            ErrType::OSError,
            &format!("Having trouble writing {}: {}", build_path(ENTRY_FILE), e),
        );
        exit(1);
    })
}
