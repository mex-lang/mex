use std::borrow::Cow;
use lalrpop_util::ErrorRecovery;
use crate::ast::*;
use crate::lexer::{LexicalError, Token};

pub trait Target {
    fn render_text(&self, text: String, level: isize);
    fn render_line(&self);
}

pub trait Transformer<'a> {
    fn visit_id(&'a self, id: &'a Id) -> Cow<'a, str>;
    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>) -> Cow<'a, str>;
    fn visit_scope(&'a self, item: &'a RefScope, level: isize);
    fn visit_global(&'a self, items: &'a Vec<RefScope>, level: isize);
    fn visit_package(&'a self, id: &'a Id, items: &'a Vec<RefScope>, level: isize);
    fn visit_model(&'a self, def: &'a ModelDefinition, level: isize);
    fn visit_model_item(&'a self, item: &'a RecordItem, level: isize);
    fn visit_scalar(&'a self, id: &'a Id, level: isize);
    fn visit_error(&'a self, error: &ErrorRecovery<usize, Token, LexicalError>, level: isize);
}

pub struct ConsoleRender {
    indent_count: isize
}

impl ConsoleRender {
    pub fn new(indent_count: isize) -> Self {
        ConsoleRender { indent_count }
    }
}

impl Target for ConsoleRender {
    fn render_text(&self, text: String, level: isize) {
        let n = level * self.indent_count;
        let indent = " ".repeat(n.try_into().unwrap_or_default());
        println!("{}{:}", indent, text);
    }

    fn render_line(&self) {
        println!();
    }
}

pub struct MexFileTransformer<'a, R: Target> {
    target: &'a R,
}

impl<'a , R: Target> MexFileTransformer<'a, R> {

    pub fn new(target: &'a R) -> Self {
        MexFileTransformer { target }
    }

    pub fn apply(&self, scope: &RefScope) {
        self.visit_scope(&scope, -1);
    }

    fn render_line(&self) {
        self.target.render_line();
    }

    fn render_text(&self, text: String, level: isize) {
        self.target.render_text(text, level);
    }
}

impl<'a, R: Target> Transformer<'a> for MexFileTransformer<'a, R> {

    fn visit_id(&'a self, id: &'a Id) -> Cow<'a, str> {
        let text = match id {
            Id::Name(ref str) => str,
            _ => "???"
        };
        Cow::Borrowed(text)
    }

    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>) -> Cow<'a, str> {
        match item_type {
            ItemType::Name(ref id) => self.visit_id(id),
            ItemType::Inline(ref model) => {
                match model {
                    ModelDefinition::Fragment(_, _, _) => unreachable!(),
                    ModelDefinition::Record(ref id, _, _) => self.visit_id(id),
                    ModelDefinition::Tuple(ref id, _, _) => self.visit_id(id),
                    ModelDefinition::Enum(ref id, _, _) => self.visit_id(id),
                    ModelDefinition::Scalar(_) => unreachable!(),
                }
            },
        }
    }

    fn visit_scope(&self, item: &RefScope, level: isize) {

        let item = item.borrow();

        match **item {
            Scope::Global(ref items) => {
                self.visit_global(items, level);
            }
            Scope::Package(ref id, ref items) => {
                self.visit_package(id, items, level);
            }
            Scope::Model(ref def) => {
                self.visit_model(def, level);
            }
            Scope::Error(ref error) => {
                self.visit_error(error, level);
            }
        }
    }

    fn visit_global(&'a self, items: &'a Vec<RefScope>, level: isize) {
        for item in items {
            self.visit_scope(item, level + 1);
        }
    }

    fn visit_package(&'a self, id: &'a Id, items: &'a Vec<RefScope>, level: isize) {
        if level < 0 {
            self.render_text(format!("package {:};", self.visit_id(id)).into(), level);

            for item in items {
                self.visit_scope(item, level + 1);
            }

            return;
        }

        self.render_line();
        self.render_text(format!("package {:} {{", self.visit_id(id)).into(), level);

        for item in items {
            self.visit_scope(item, level + 1);
        }

        self.render_text(format!("}}").into(), level);
    }

    fn visit_model(&self, def: &ModelDefinition, level: isize) {

        match def {
            ModelDefinition::Scalar(ref id) => {
                self.visit_scalar(id, level);
            }
            ModelDefinition::Record(ref id, ref items, ref _params) => {
                self.render_line();
                self.render_text(format!("model {:} {{", self.visit_id(id)).into(), level);

                for item in items {
                    self.visit_model_item(item, level + 1);
                }

                self.render_text(format!("}}").into(), level);
            },
            ModelDefinition::Fragment(ref id, ref items, ref _params) => {
                self.render_line();
                self.render_text(format!("fragment {:} {{", self.visit_id(id)).into(), level);

                for item in items {
                    self.visit_model_item(item, level + 1);
                }

                self.render_text(format!("}}").into(), level);
            }
            ModelDefinition::Enum(ref id, ref items, ref _params) => {
                self.render_line();
                self.render_text(format!("enum {:} {{", self.visit_id(id)).into(), level);

                for item in items {

                    match item {
                        EnumItem::Item(ref id) => {
                            self.render_text(format!("{:},", self.visit_id(id)), level + 1);
                        },
                        EnumItem::Record(ref id, ref type_id) => {
                            self.render_text(format!("{:}({{{:}}}),", self.visit_id(id), self.visit_item_type(type_id)), level + 1);
                        },
                        EnumItem::Tuple(ref id, ref type_id) => {
                            self.render_text(format!("{:}({:}),", self.visit_id(id), self.visit_item_type(type_id)), level + 1);
                        }
                    }
                }

                self.render_text(format!("}}").into(), level);
            },
            ModelDefinition::Tuple(ref id, ref items, ref _params) => {
                self.render_line();

                let items = items.iter().map(|item| match item {
                    TupleItem::Item(ref type_id) => self.visit_item_type(type_id).into_owned(),
                    TupleItem::NamedItem(ref id, ref type_id) => format!("{:}: {:}", self.visit_id(id), self.visit_item_type(type_id)),
                }).collect::<Vec<_>>().join(", ");

                self.render_text(format!("model {:}({:})", self.visit_id(id), items), level);
            }
        }
    }

    fn visit_model_item(&self, item: &RecordItem, level: isize) {
        match item {
            RecordItem::Item(ref id, ref type_id) => {
                let text = format!("{:}: {:},", self.visit_id(id), self.visit_item_type(type_id));
                self.render_text(text.into(), level);
            }
            RecordItem::Spread(ref type_id) => {
                let text = format!("... {:},", self.visit_item_type(type_id));
                self.render_text(text.into(), level);
            }
        }
    }

    fn visit_scalar(&self, id: &Id, level: isize) {
        self.render_line();
        self.render_text(format!("scalar {:};", self.visit_id(id)).into(), level);
    }

    fn visit_error(&self, error: &ErrorRecovery<usize, Token, LexicalError>, level: isize) {
        self.render_line();
        self.render_text(format!("error: {:?}", error).into(), level);
    }
}