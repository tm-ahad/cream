use crate::component::Component;
use crate::component::final_build_string;
use crate::component::std_lib_path;
use oxc_allocator::CloneIn;
use oxc_codegen::Codegen;
use oxc_codegen::CodegenOptions;
use oxc_codegen::CommentOptions;
use oxc_minifier::Minifier;
use oxc_minifier::MinifierOptions;
use oxc_semantic::SemanticBuilder;
use oxc_transformer::TransformOptions;
use oxc_transformer::Transformer;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::helpers::build_source::build_import;
use oxc_allocator::Allocator;
use oxc_ast::AstBuilder;
use oxc_ast::ast::Statement;
use oxc_parser::Parser;
use oxc_span::SourceType;
use crate::std_err::ErrType;
use crate::component::format_oxc_diag;
use crate::std_err::StdErr;
use std::fs::read_to_string;
use std::path::Path;

impl<'a> Component<'a> {
    pub fn transpile(&mut self) {
        let script: String = read_to_string(self.name.clone()).expect(&format!("{} not found", self.name));
        let comp = Component::new(script, self.name.clone(), self.router_map, self.dep_graph);
        let render = comp.html_rendering_script().unwrap();

        let mut rng = ThreadRng::default();
        let comp_id = rng.next_u64();

        let script_trimmed = final_build_string(render, comp_id);

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true);
        let parsed = Parser::new(&allocator, &script_trimmed, source_type).parse();
        let mut program = parsed.program;

        let allocator = Allocator::default();
        let ast = AstBuilder::new(&allocator);
        let mut new_program = oxc_allocator::Vec::new_in(&allocator);

        program.body.retain(|stmt| {
            match stmt {
                Statement::ImportDeclaration(src) => {
                    let source = src.clone_in(&allocator);
                    
                    if source.source.value.starts_with("@") {
                        let resolved = if source.source.value.starts_with("@std:") {
                            let name = &source.source.value["@std:".len()..].to_string();
                            self.dep_graph.add_std_lib(name);
                            std_lib_path(name)
                        } else {
                            build_import(&source.source.value["@".len()..], self.router_map)
                        };  
                                        
                        let new_import = ast.import_declaration(
                            source.span, 
                            source.specifiers.clone_in(&allocator), 
                            ast.string_literal(source.span, ast.str(&resolved), Some(ast.str(&resolved))),
                            source.phase, 
                            source.with_clause.clone_in(&allocator), 
                            source.import_kind
                        );

                        new_program.insert(0, Statement::ImportDeclaration(oxc_allocator::Box::new_in(new_import, &allocator)));
                    }

                    false
                },
                _ => {
                    new_program.push(stmt.clone_in(&allocator));
                    true
                }

            }
        });

        let mut new_prog = ast.program(
            program.span, 
            program.source_type, 
            "", 
            program.comments, 
            program.hashbang, 
            program.directives,
            new_program
        );

        let min_opt = MinifierOptions::default();
        let min = Minifier::new(min_opt);

        let allocator = Allocator::default();
        let transform_options = TransformOptions::default();
        
        let semantic_ret = SemanticBuilder::new()
            .build(&new_prog);

        let scoping = semantic_ret.semantic.into_scoping();
        let transformer = Transformer::new(
            &allocator,
            Path::new(&self.name),
            &transform_options,
        );
        
        for err in parsed.diagnostics {
            StdErr::exec(ErrType::SyntaxError, &format_oxc_diag(&err, self.name.clone()));
        }
    
        let _ = transformer.build_with_scoping(scoping, &mut new_prog);
        let _ = min.minify(&allocator, &mut new_prog);
        let opt_script = Codegen::new()
            .with_options(CodegenOptions {
                minify: !cfg!(debug_assertions),
                single_quote: true,
                comments: CommentOptions::disabled(),
                ..Default::default()
            })
            .with_source_type(SourceType::mjs())
            .build(&new_prog)
            .code;

        self.out = opt_script;
    }
}

