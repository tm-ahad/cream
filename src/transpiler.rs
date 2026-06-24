use crate::component::Component;
use crate::component::RenderReturn;
use crate::component::cream_component;
use crate::component::cream_dom_name;
use crate::component::special_trim;
use crate::component::std_lib_path;
use crate::helpers::javascript::javascript_function::javascript_function;
use oxc_allocator::CloneIn;
use oxc_ast::ast::Function;
use oxc_codegen::Codegen;
use oxc_codegen::CodegenOptions;
use oxc_codegen::CommentOptions;
use oxc_minifier::Minifier;
use oxc_minifier::MinifierOptions;
use oxc_semantic::SemanticBuilder;
use oxc_span::Span;
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

pub fn final_build_string(render: RenderReturn, comp_id: u64) -> String {
    special_trim(format!("
            let self; let onRender=function(){{}};
            function {}(params, elements={{}}) {{{}; 
                {}; onRender();
                return {}
            }}; {}
            export {{{} as {}, mount}};
        ",  cream_component(comp_id),
            render.script, render.rendering_script, 
            cream_dom_name(render.root_dom_id),
            javascript_function(String::from("mount"), 
                &format!(
                    "document.body.appendChild({}({{}}, elements));",
                    cream_component(comp_id)
                ),
                vec!["elements".to_string()]
            ),
            cream_component(comp_id),
            render.comp_name
        ))
}

impl<'a> Component<'a> {
    pub fn handle_comp_declr<'b>(
        &mut self,
        stmt: &mut Function,
        new_program: &mut oxc_allocator::Vec<'b, Statement<'b>>,
        allocator: &'b Allocator,
        ast: &AstBuilder<'b>,
    ) {
        if let Some(body) = &mut stmt.body {
            body.statements.retain(|stmt| {
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
                    Statement::LabeledStatement(label) => {
                        if label.label.name.to_string() == "global" {
                            new_program.push(label.body.clone_in(&allocator));
                            false
                        } else {
                            true
                        }
                    }
                    _ => true
                }
            });
        }

    }

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
        let program = parsed.program;

        let allocator = Allocator::default();
        let ast = AstBuilder::new(&allocator);
        let mut new_program = oxc_allocator::Vec::new_in(&allocator);

        for mut stmt in program.body {
            match &mut stmt {
                Statement::LabeledStatement(label) => {
                    if label.label.name.to_string() == "global" {
                        new_program.push(label.body.clone_in(&allocator));
                    }
                },
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
                },
                Statement::FunctionDeclaration(func) => {
                     if let Some(name) = func.name() && name.to_string() == cream_component(comp_id) {
                         let mut bind = func.clone_in(&allocator);
                         self.handle_comp_declr(&mut bind, &mut new_program, &allocator, &ast);

                         println!("{:?}\n\n\n", bind);
                         new_program.push(Statement::FunctionDeclaration(bind));
                    } else {
                        new_program.push(stmt);
                    }
                }
                _ => {
                    new_program.push(stmt.clone_in(&allocator));
                }

            }
        }

        let mut new_prog = ast.program(
            program.span, 
            program.source_type, 
            "", 
            program.comments, 
            None, 
            oxc_allocator::Vec::new_in(&allocator),
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

