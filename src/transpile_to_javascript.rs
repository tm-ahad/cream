
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use oxc_codegen::Codegen;

pub fn transpile_script(lang: &str, script: &mut String) {
    if lang == "js" {
    } else if lang == "ts" {
        let transpiled_code = transpile_typescript(script);
        *script = transpiled_code;
    } else {
        panic!("Invalid language: {}", lang);
    }
}

fn transpile_typescript(ts_code: &str) -> String {
    let source_type = SourceType::from_path("dih.ts").unwrap();
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, ts_code, source_type)
        .with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() })
        .parse();
    let program = ret.program;
    
    Codegen::new().build(&program).code
}
