use mex_lang::ast::Source;

fn main() {
    let source = Source::from_file("examples/empty_package.mex");
    let code = source.read().unwrap();
    let ast = source.parse(&code).unwrap();

    println!("{:?}", ast);
}