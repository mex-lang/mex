use std::borrow::Cow;
use lalrpop_util::lalrpop_mod;
use crate::ast::{Error, RefScope, Source};
use crate::lexer::Lexer;
use crate::mex::PackageParser;

pub mod ast;
pub mod lexer;
pub mod transform;

lalrpop_mod!(pub mex);

pub struct Compiler<'a>(&'a Source, Cow<'a, str>, &'a RefScope<'a>);

impl<'a> Compiler<'a> {

    pub fn new(source: &'a Source, global: &'a RefScope<'a>) -> Result<'a, Self> {
        let code = source.read()?;
        Ok(Self(source, code, global))
    }

    fn make_lexer(&'a self) -> Lexer<'a> {
        Lexer::new(&self.1)
    }

    fn make_parser(&'a self) -> PackageParser {
        PackageParser::new()
    }

    pub fn make_ast(&'a self) -> Result<'a, RefScope<'a>> {
        let lexer = self.make_lexer();
        let parser = self.make_parser();
        let ast = parser.parse(self.0, self.2, lexer)?;
        Ok(ast)
    }
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::ast::Scope;
    use crate::transform::{MexLangTransformer, StringRender};

    #[test_case("package name;"; "root package")]
    #[test_case("package name {\n}"; "empty package")]
    #[test_case("package name {\n}\n\npackage name {\n}"; "double package")]
    #[test_case("model Point()"; "empty tuple model")]
    #[test_case("package name;\n\nmodel Point()"; "model in package")]
    #[test_case("scalar s1;\nscalar s1;"; "double scalar")]

    #[test_case("model Test(Int, Int)"; "named tuple")]
    #[test_case("model Point()"; "empty tuple")]
    #[test_case("model (Int, Int)"; "inline tuple")]
    #[test_case("model ()"; "inline empty tuple")]
    #[test_case("model (x: Int, y: Int)"; "inline tuple with names")]

    #[test_case("model Point(Int, (Int, Int))"; "inline nested tuple")]
    #[test_case("model Point(x: Int, y: (Int, Int))"; "inline nested tuple with names")]

    #[test_case("model Test {\n    x: Int\n    y: Int\n    ... Test\n}"; "named record")]
    #[test_case("model {\n    ... Test\n    x: Int\n}"; "inline record")]

    #[test_case("enum Test {\n    Int\n}"; "named enum")]
    #[test_case("enum {\n    Int\n}"; "inline enum")]
    #[test_case("enum Status {\n    Test(Int)\n}"; "nested tuple enum")]
    #[test_case("enum Status {\n    Test {\n        x: Int\n    }\n}"; "nested record enum")]
    #[test_case("enum Status {\n    Sub enum {\n        Test\n    }\n}"; "nested inline enum")]


    fn check(code: &str) {
        let global = Scope::Global(vec![]).into();
        let source = Source::from_str(code);
        let compiler = Compiler::new(&source, &global).unwrap();
        let ast = compiler.make_ast().unwrap();

        let render = StringRender::new();
        let transformer = MexLangTransformer::new(&render);
        transformer.apply(&ast);
        let result = render.as_string(4);

        assert_eq!(&result, code)
    }
}
