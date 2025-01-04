use mex_lang::ast::{Scope, Source};

fn main() {
    let global = Scope::Global(vec![]).into();

    let source = Source::from_file("examples/empty_package.mex");
    let code = source.read().unwrap();
    let ast = source.parse(&code, &global).unwrap();

    println!("ast: {:?}", ast);
    println!("global: {:?}", global);
}