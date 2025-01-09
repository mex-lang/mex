use mex_lang::ast::{Scope, Source};
use mex_lang::Compiler;
use mex_lang::transform::{ConsoleRender, MexLangTransformer};

fn main() {
    let global = Scope::Global(vec![]).into();
    let source = Source::from_file("examples/empty_package.mex");
    let compiler = Compiler::new(&source, &global).unwrap();
    let ast = compiler.make_ast().unwrap();

    println!("global: {:?}", global);

    let render = ConsoleRender::new(4);
    let transformer = MexLangTransformer::new(&render);
    transformer.apply(&ast);
}