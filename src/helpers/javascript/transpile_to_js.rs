use std::path::Path;

use oxc_allocator::{Allocator};
use oxc_codegen::{Codegen, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer};
use crate::component::format_oxc_diag;
use crate::std_err::{ErrType, StdErr};

pub fn transpile_to_js(src: &str, f_name: &str, opt: CodegenOptions) -> String {
    let alloc = Allocator::default();
    let mut parsed = Parser::new(
        &alloc, src, 
        SourceType::ts()
    ).parse();

    for err in parsed.diagnostics {
        StdErr::exec(ErrType::SyntaxError, &format_oxc_diag(&err, f_name.to_string()));
    }

    let semantic_ret = SemanticBuilder::new()
            .build(&parsed.program);

    let scoping = semantic_ret.semantic.into_scoping();
    let options = TransformOptions::default();
    let _ = Transformer::new(
        &alloc,
        Path::new("router.ts"),
        &options,
    ).build_with_scoping(scoping, &mut parsed.program);

    Codegen::new()
        .with_options(opt)
        .with_source_type(SourceType::mjs())
        .build(&parsed.program)
        .code
}
