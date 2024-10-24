use crate::component::Component;
use crate::component_map::ComponentMap;
use crate::consts::{FROM_TOKEN, IC_TOKEN, IC_TOKEN_LEN, NEW_LINE};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::pass::pass;

pub fn import_component(app: &str, f_name: String, component_map: &mut ComponentMap<'_>) -> Vec<Component> {
    let mut components = Vec::new();

    match app.find(IC_TOKEN) {
        Some(a) => {
            let n = read_until(app, a+IC_TOKEN_LEN, NEW_LINE, &f_name, ComponentPart::Unknown);
            let r_c_meta_data = &app[a+IC_TOKEN_LEN..n];

            if let Some(i) = r_c_meta_data.find(FROM_TOKEN) {
                let (mut component_name, _) = r_c_meta_data.split_at(i);
                component_name = component_name.trim();

                components.push(component_map.get(f_name.clone(), component_name.to_string()).clone());
            }
        },
        None => pass()
    }

    components
}

