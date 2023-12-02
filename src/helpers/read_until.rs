use crate::helpers::component_part::ComponentPart;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;

pub fn read_until(
    s: &str,
    start: usize,
    pat: &str,
    f_name: &str,
    _where: ComponentPart
) -> usize {
    let ind = s[start..].find(pat);

    match ind {
        Some(i) => i+start,
        None => {
            StdErr::exec(
                SyntaxError,
                &format!("{pat} expected in {} ({f_name})", _where.to_raw_string())
            );
            todo!()
        }
    }
}
