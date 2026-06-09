use crate::helpers::javascript::{javascript_function::javascript_function, javascript_function_call::javascript_function_call, javascript_expr_in_bracket::javascript_expr_in_bracket};

pub fn javascript_anonymus_call(body: String) -> String {
    javascript_function_call(
        &javascript_expr_in_bracket(
            javascript_function(String::new(), body, vec![])
        ),
        vec![]
    )
}
