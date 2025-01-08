use std::borrow::Cow;
use lalrpop_util::ErrorRecovery;
use crate::ast::{ModelItemDefinition, Id, ItemType, ModelDefinition, RefScope, Scope};
use crate::lexer::{LexicalError, Token};

pub trait Transformer<'a> {
    fn render_text(&'a self, text: Cow<'a, str>, level: isize);
    fn render_line(&'a self);

    fn visit_id(&'a self, id: &'a Id) -> Cow<'a, str>;
    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>) -> Cow<'a, str>;
    fn visit_scope(&'a self, item: &'a RefScope, level: isize);
    fn visit_global(&'a self, items: &'a Vec<RefScope>, level: isize);
    fn visit_package(&'a self, id: &'a Id, items: &'a Vec<RefScope>, level: isize);
    fn visit_model(&'a self, def: &'a ModelDefinition, level: isize);
    fn visit_model_item_def(&'a self, item: &'a ModelItemDefinition, level: isize);
    fn visit_scalar(&'a self, id: &'a Id, level: isize);
    fn visit_error(&'a self, error: &ErrorRecovery<usize, Token, LexicalError>, level: isize);
}

pub struct ConsoleRenderer {
    indent_count: isize
}

impl ConsoleRenderer {
    pub fn new(indent_count: isize) -> Self {
        ConsoleRenderer { indent_count }
    }

    pub fn apply(&self, scope: &RefScope) {
        self.visit_scope(&scope, -1);
    }
}

impl<'a> Transformer<'a> for ConsoleRenderer {

    fn render_text(&self, text: Cow<'_, str>, level: isize) {
        let n = level * self.indent_count;
        let indent = " ".repeat(n.try_into().unwrap_or_default());
        println!("{}{:}", indent, text);
    }

    fn render_line(&'a self) {
        println!();
    }

    fn visit_id(&'a self, id: &'a Id) -> Cow<'a, str> {
        let text = match id {
            Id::Name(ref str) => str,
            _ => "???"
        };
        Cow::Borrowed(text)
    }

    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>) -> Cow<'a, str> {
        match item_type {
            ItemType::Model(ref id) => self.visit_id(id),
            _ => Cow::Borrowed("???")
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

    fn visit_model_item_def(&self, item: &ModelItemDefinition, level: isize) {
        match item {
            ModelItemDefinition::Item(ref id, ref type_id) => {
                let text = format!("{:}: {:},", self.visit_id(id), self.visit_item_type(type_id));
                self.render_text(text.into(), level);
            }
            ModelItemDefinition::Spread(ref type_id) => {
                let text = format!("... {:},", self.visit_item_type(type_id));
                self.render_text(text.into(), level);
            }
        }
    }

    fn visit_model(&self, def: &ModelDefinition, level: isize) {

        match def {
            ModelDefinition::Scalar(ref id) => {
                self.visit_scalar(id, level);
            }
            ModelDefinition::Record(ref id, ref items, ref params) => {
                self.render_line();
                self.render_text(format!("model {:} {{", self.visit_id(id)).into(), level);

                for item in items {
                    self.visit_model_item_def(item, level + 1);
                }

                self.render_text(format!("}}").into(), level);
            },
            ModelDefinition::Enum(ref id, ref fields, ref params) => {
            },
            ModelDefinition::Tuple(ref id, ref fields, ref params) => {
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