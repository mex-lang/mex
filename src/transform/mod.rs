use lalrpop_util::ErrorRecovery;
use crate::ast::*;
use crate::lexer::{LexicalError, Token};

mod string_render;
mod console_render;
mod mex_lang_transformer;

pub use string_render::StringRender;
pub use console_render::ConsoleRender;
pub use mex_lang_transformer::MexLangTransformer;

pub enum TextToken {
    None,

    Space,
    NewLine,

    LineIndent,
    IncIndent,
    DecIndent,

    Text(String),
    //Template(String, HashMap<String, String>),
}

pub trait Target<Token> {
    fn render(&self, token: Token);
}

pub trait Transformer<'a> {
    fn visit_id(&'a self, id: &'a Id);
    fn visit_literal(&self, literal: &Literal);
    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>);
    fn visit_model_params(&self, params: &Vec<ModelParam>);
    fn visit_model_params_def(&self, params: &Vec<ModelParamDefinition>);
    fn visit_scope(&'a self, item: &'a RefScope, is_root: bool);
    fn visit_global(&'a self, items: &'a Vec<RefScope>);
    fn visit_package(&'a self, id: &'a Id, items: &'a Vec<RefScope>, is_root: bool);
    fn visit_model(&'a self, def: &'a ModelDefinition);
    fn visit_header_model(&self, keyword: &str);
    fn visit_record_model(&self, id: &Id, items: &Vec<RecordItem>, _params: &Vec<ModelParamDefinition>);
    fn visit_tuple_model(&self, id: &Id, items: &Vec<TupleItem>, _params: &Vec<ModelParamDefinition>);
    fn visit_enum_model(&self, id: &Id, items: &Vec<EnumItem>, _params: &Vec<ModelParamDefinition>);
    fn visit_model_item(&'a self, item: &'a RecordItem);
    fn visit_enum_item(&self, item: &EnumItem);
    fn visit_scalar(&'a self, id: &'a Id);
    fn visit_error(&'a self, error: &ErrorRecovery<usize, Token, LexicalError>);
}

