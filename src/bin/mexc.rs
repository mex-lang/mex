use mex_lang::ast::{Scope, Source};
use mex_lang::Compiler;
use mex_lang::transform::{ConsoleRenderer};

fn main() {
    let global = &Scope::Global(vec![]).into();
    let source = Source::from_file("examples/empty_package.mex");
    let compiler = Compiler::new(&source, global).unwrap();
    let ast = compiler.make_ast().unwrap();

    println!("global: {:?}", global);

    let render = ConsoleRenderer::new(4);
    render.apply(&ast);
}