use crate::std_err::ErrType::ConfigError;
use crate::std_err::StdErr;
use std::collections::HashMap;

pub fn get_prop(h: HashMap<String, String>, key: &str) -> String {
    return match h.get(key) {
        Some(a) => a.clone(),
        None => {
            let err = StdErr::new(ConfigError,
                                  &*format!("{key} not found on config"));
            err.exec();

            todo!()
        }
    }
}
