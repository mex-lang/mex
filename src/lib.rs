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
    use ast::Scope::{Global, Package};

    #[test]
    fn one_root_package() {
        let global = &Global(vec![]).into();
        let source = Source::from_str("package main;");
        let compiler = Compiler::new(&source, global).unwrap();
        let ast = compiler.make_ast().unwrap();

        let expected:RefScope = Package("main".into(), vec![]).into();

        assert_eq!(ast, expected);
    }

    #[test]
    fn one_nested_package() {
        let global = &Global(vec![]).into();
        let source = Source::from_str("package main {}");
        let compiler = Compiler::new(&source, global).unwrap();
        let ast = compiler.make_ast().unwrap();

        let expected:RefScope = Global(vec![
            Package("main".into(), vec![]).into(),
        ]).into();

        assert_eq!(ast, expected);
    }

    #[test]
    fn root_and_nested_package() {
        let global = &Global(vec![]).into();
        let source = Source::from_str("package main; \n package a {} \n package b {}");
        let compiler = Compiler::new(&source, global).unwrap();
        let ast = compiler.make_ast().unwrap();

        let expected:RefScope = Package("main".into(), vec![
            Package("a".into(), vec![]).into(),
            Package("b".into(), vec![]).into(),
        ]).into();

        assert_eq!(ast, expected);
    }

    #[test]
    fn nested_package() {
        let global = &Global(vec![]).into();
        let source = Source::from_str("package a {} \n package b {}");
        let compiler = Compiler::new(&source, global).unwrap();
        let ast = compiler.make_ast().unwrap();

        let expected:RefScope = Global(vec![
            Package("a".into(), vec![]).into(),
            Package("b".into(), vec![]).into(),
        ]).into();

        assert_eq!(ast, expected);
    }
}
