use std::path::Path;

use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions};
use oxc_parser::Parser;
use oxc_span::SourceType;
use crate::component::format_oxc_diag;
use crate::std_err::{ErrType, StdErr};


pub fn transpile_to_js(src: &str, f_name: &str, opt: CodegenOptions) -> String {
    let alloc = Allocator::default();
    let parsed = Parser::new(
        &alloc, src, 
        SourceType::from_path(Path::new(f_name)).unwrap()
    ).parse();

    for err in parsed.diagnostics {
        StdErr::exec(ErrType::SyntaxError, &format_oxc_diag(&err, f_name.to_string()));
    }

    Codegen::new()
        .with_options(opt)
        .build(&parsed.program)
        .code
}
