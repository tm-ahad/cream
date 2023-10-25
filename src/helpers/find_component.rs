use crate::component::Component;

pub fn find_component_by_name(comps: &Vec<Component>, name: String) -> Option<&Component> {
    for comp in comps {
        if comp.name == name {
            return Some(comp)
        }
    }

    None
}

