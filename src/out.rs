use crate::component::{Component, cream_dom_name};
use crate::dsp_map::DspMap;
use crate::std_err::{ErrType, StdErr};
use std::fs::{self, read_to_string};
use oxc_diagnostics::OxcDiagnostic;
use oxc_minifier::{Minifier, MinifierOptions};
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_codegen::{Codegen, CodegenOptions, CommentOptions};
use oxc_transformer::{TransformOptions, Transformer};
use std::process::exit;
use std::path::Path;


fn write_file(path: &str, contents: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;
    Ok(())
}

fn format_oxc_diag(diag: &OxcDiagnostic, src: String) -> String {
    format!("{} at {}:{}", diag.message, src, diag.labels.clone().unwrap()[0].offset())
}

pub fn out(
    path: &str,
    src: &str,
    html: String,
    script: String,
    config: &DspMap
) {
    let head_prefix = format!("./{}", config.expect("head_prefix"));
    let head = read_to_string(head_prefix.clone())
        .unwrap_or_else(|e| panic!("{head_prefix}: {e}"));

    let comp = Component::new(String::new(), html, String::new(), String::new());
    let (html, id) = comp.html_rendering_script().unwrap();
    let min_opt = MinifierOptions::default();
    let min = Minifier::new(min_opt);

    let unopt_script = format!("var elements = {{}};
        var self; var onRender = function(){{}};
        {script};{html};
        document.body.appendChild({});
        onRender();
    ", cream_dom_name(id));

    let source_type = SourceType::from_path(&format!("{src}.ts")).unwrap();
    let allocator = Allocator::default();
    let ret = Parser ::new(&allocator, &unopt_script, source_type)
        .with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() })
        .parse();

    let mut program = ret.program;
    let errors = ret.errors;
    let transform_options = TransformOptions::default();
    
    for err in errors.iter() {
        StdErr::exec(ErrType::SyntaxError, &format_oxc_diag(err, src.to_string()));
    }
    
    if !errors.is_empty() {exit(1)}
    let semantic_ret = SemanticBuilder::new()
        .build(&program);

    let scoping = semantic_ret.semantic.into_scoping();
    let transformer = Transformer::new(
        &allocator,
        Path::new("<unkown>.js"),
        &transform_options,
    );
    
    let _ = transformer.build_with_scoping(scoping, &mut program);

    let _ = min.minify(&allocator, &mut program);
    let opt_script = Codegen::new()
        .with_options(CodegenOptions {
            minify: !cfg!(debug_assertions),
            single_quote: true,
            comments: CommentOptions::disabled(),
            ..Default::default()
        })
        .with_source_type(SourceType::mjs())
        .build(&program)
        .code;

    write_file(&path,
        &format!(
            "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta name=\"description\" content=\"{}\">
    <meta name=\"keywords\" content=\"{}\">
    <meta name=\"author\" content=\"{}\">
    <title>{}</title>
    {head}
</head>
<body>
<script>{opt_script}</script>
<body>
</html>
",
            config.expect("description"),
            config.expect("keywords"),
            config.expect("author"),
            config.expect("title")
        ),
    )
    .unwrap_or_else(|e| panic!("{}", e));
}
