use lalrpop_util::lalrpop_mod;
use crate::ast::{Error, Scope, Source};
use crate::lexer::Lexer;
use crate::mex::PackageParser;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub mex);

impl<'input> Source {
    pub fn parse<'code: 'input>(&'input self, code: &'code str) -> Result<Box<Scope<'input>>, Error<'input>> {
        let lexer = Lexer::new(code);
        let parser = PackageParser::new();
        let ast = parser.parse(&self, lexer)?;
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
