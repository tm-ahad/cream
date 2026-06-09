use crate::helpers::javascript::javascript_assign::javascript_assign;
use crate::helpers::javascript::javascript_function_call::javascript_function_call;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;

pub fn template(
    html: &mut String,
    script: &mut String,
    f_name: &str,
) {

    let tok_size: usize = "@render_by_id".len();
    while let Some(a) = html.find("@render_by_id") {
        if html.chars().nth(a+tok_size) == Some(':') {
            let ind = read_until(html, a+1, " ", f_name, ComponentPart::Script);
            let id = &html[a+tok_size+1..ind];

            let content_ind = read_until(html, ind, ";", f_name, ComponentPart::Script);
            
            script.push_str(&javascript_assign(
                format!("{}.innerHTML",
                    javascript_function_call("document.getElementById", vec![id.trim().to_string()])
                ), 
                html[ind..content_ind].to_string()
            ));
            html.replace_range(a..content_ind, "");
        } else {
            html.replace_range(a..a+tok_size, "");
        }
    }
}
