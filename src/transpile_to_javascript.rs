use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsConfig};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::ecma::transforms::base::hygiene::hygiene;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::transforms::base::fixer::fixer;
use swc_core::common::{SourceMap, GLOBALS, Mark};
use swc_core::ecma::codegen::{Config, Emitter};
use swc_core::common::{BytePos, Globals};
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::typescript::strip;
use swc_core::ecma::visit::FoldWith;
use swc_core::common::sync::Lrc;

pub fn transpile_script(lang: &str, script: &mut String) {
    if lang == "js" {
    } else if lang == "ts" {
        let transpiled_code = transpile_typescript(script).unwrap_or_else(|err| {
            panic!("Failed to transpile TypeScript: {}", err);
        });

        *script = transpiled_code;
    } else {
        panic!("Invalid language: {}", lang);
    }
}

fn transpile_typescript(ts_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cm: Lrc<SourceMap> = Default::default();
    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: false,
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::new(ts_code, BytePos(0), BytePos(ts_code.len() as u32)),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    let program = parser
        .parse_program()
        .map_err(|e| panic!("{e:?}"))
        .expect("failed to parse program.");

    let globals = Globals::default();
    let ret = GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        let program = program
            .fold_with(&mut resolver(unresolved_mark, top_level_mark, true))
            .fold_with(&mut strip(top_level_mark))
            .fold_with(&mut hygiene())
            .fold_with(&mut fixer(Some(&comments)));

        let mut buf = vec![];
        {
            let mut config = Config::default();

            config.minify = true;
            config.ascii_only = true;
            config.inline_script = true;

            let mut emitter = Emitter {
                cfg: config,
                cm: cm.clone(),
                comments: Some(&comments),
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_program(&program).unwrap();
        }

        Ok(String::from_utf8(buf)?)
    });

    ret
}
