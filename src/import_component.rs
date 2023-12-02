use crate::component::{Component, component};
use crate::component_args::ComponentArgs;
use crate::consts::{FROM_TOKEN, FROM_TOKEN_LEN, IC_TOKEN, IC_TOKEN_LEN, NEW_LINE};
use crate::pass::pass;

pub fn import_component(app: &mut str, component_args: &ComponentArgs) -> Vec<Component> {
    let mut ret = Vec::new();

    match app.find(IC_TOKEN) {
        Some(a) => {
            let mut n = a+IC_TOKEN_LEN;

            while &app[n..n+1] != NEW_LINE {
                n += 1;
            }

            let r_c_meta_data = &app[a+IC_TOKEN_LEN..n];

            if let Some(i) = r_c_meta_data.find(FROM_TOKEN) {
                let (mut cn, mut p) = r_c_meta_data.split_at(i);
                p = p[FROM_TOKEN_LEN+1..].trim();
                cn = cn.trim();

                ret.push(component(
                    p,
                    cn,
                    component_args.transpile_command,
                    component_args.config
                ));
            }
        },
        None => pass()
    }

    ret
}

