use crate::consts::IMP_STATE_SIGN;

pub fn imp_sign(s: String) -> String {
    format!("\n{}{s}", IMP_STATE_SIGN)
}
