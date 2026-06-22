
pub fn javascript_function(name: String, body: &str, params: Vec<String>) -> String {
    let params_joined = params.join(",");
    format!("function {name}({params_joined}) {{ ;{body}; }}")
}
