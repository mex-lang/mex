use lalrpop_util::lalrpop_mod;
use crate::ast::{Error, RefScope, Source};
use crate::lexer::Lexer;
use crate::mex::PackageParser;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub mex);

impl<'input> Source {
    pub fn parse<'code: 'input, 'global: 'input>(
        &'input self,
        code: &'code str,
        global: &'global RefScope<'input>
    ) -> ParseResult<'input> {

        let lexer = Lexer::new(code);
        let parser = PackageParser::new();
        let ast = parser.parse(&self, global, lexer)?;
        Ok(ast)
    }
}

pub type ParseResult<'input> = Result<RefScope<'input>, Error<'input>>;

#[cfg(test)]
mod tests {
    use super::*;
}
