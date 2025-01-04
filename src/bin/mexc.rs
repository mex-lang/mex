use mex_lang::ast::{Scope, Source};
use mex_lang::Compiler;

fn main() {
    let global = &Scope::Global(vec![]).into();
    let source = Source::from_file("examples/empty_package.mex");
    let compiler = Compiler::new(&source, global).unwrap();
    let ast = compiler.make_ast().unwrap();

    println!("ast: {:?}", ast);
    println!("global: {:?}", global);
}