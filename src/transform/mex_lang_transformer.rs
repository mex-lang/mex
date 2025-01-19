use std::cell::RefCell;
use lalrpop_util::ErrorRecovery;
use crate::ast::*;
use crate::lexer::{LexicalError, Token};
use crate::transform::{Target, TextToken, Transformer};

pub struct MexLangTransformer {
    tokens: RefCell<Vec<TextToken>>,
}

impl MexLangTransformer {
    pub fn new() -> Self {
        MexLangTransformer {
            tokens: RefCell::new(Vec::new()),
        }
    }

    pub fn apply<R: Target<TextToken>>(self, scope: &RefScope, render: &R) {
        self.visit_scope(&scope, true);

        self.tokens
            .into_inner().into_iter()
            .for_each(|t| {render.render(t)});
    }

    fn render(&self, token: TextToken) {
        self.tokens.borrow_mut().push(token);
    }
}

impl Transformer<'_> for MexLangTransformer {

    fn visit_id(&self, id: &Id) {
        let token = match id {
            Id::Name(ref str) => TextToken::Text(str.to_string()),
            Id::Index(ref _index) => todo!(),
            Id::Branch(ref _str, ref _branch) => todo!(),
            Id::Inline => TextToken::None,
        };

        self.render(token);
    }

    fn visit_literal(&self, literal: &Literal) {
        match literal {
            Literal::String(ref str) => self.render(TextToken::Text(str.to_string())),
            Literal::Number(ref str) => self.render(TextToken::Text(str.to_string())),
        }
    }

    fn visit_item_type(&self, item_type: &ItemType) {
        match item_type {
            ItemType::Model(ref id, ref params) => {
                self.visit_id(id);
                self.visit_model_params(params);
            },
            ItemType::Inline(ref model) => {
                match model {
                    ModelDefinition::Fragment(_, _, _) => unreachable!(),
                    ModelDefinition::Alias(_, _, _) => unreachable!(),
                    ModelDefinition::Scalar(_) => unreachable!(),
                    ModelDefinition::Record(ref id, ref items, params) => {
                        self.visit_record_model(id, items, params);
                    },
                    ModelDefinition::Tuple(ref id, ref items, params) => {
                        self.visit_tuple_model(id, items, params);
                    },
                    ModelDefinition::Enum(ref id, ref items, params) => {
                        self.visit_enum_model(id, items, params);
                    },
                }
            },
        }
    }

    fn visit_model_params(&self, params: &Vec<ModelParam>) {

        let mut generics: Vec<&ItemType> = vec!();
        let mut metadata: Vec<(&Id, &Literal)> = vec!();

        for param in params {
            match param {
                ModelParam::Generic(ref item_type) => {
                    generics.push(item_type);
                },
                ModelParam::Metadata(ref name, ref value) => {
                    metadata.push((name, value));
                },
            }
        }

        if !generics.is_empty() {
            self.render(TextToken::Text("<".to_string()));
            for (i, item_type) in generics.iter().enumerate() {
                self.visit_item_type(item_type);
                if i < generics.len() - 1 {
                    self.render(TextToken::Text(",".to_string()));
                }
            }
            self.render(TextToken::Text(">".to_string()));
        }

        if !metadata.is_empty() {
            self.render(TextToken::Text("[".to_string()));
            for (i, (name, value)) in metadata.iter().enumerate() {
                self.visit_id(name);
                self.render(TextToken::Text("=".to_string()));
                self.visit_literal(value);
                if i < metadata.len() - 1 {
                    self.render(TextToken::Text(",".to_string()));
                }
            }
            self.render(TextToken::Text("]".to_string()));
        }
    }

    fn visit_model_params_def(&self, params: &Vec<ModelParamDefinition>) {

        let mut generics: Vec<(&Id, &Option<ItemType>)> = vec!();
        let mut metadata: Vec<(&Id, &ItemType, &Option<Literal>)> = vec!();
        let mut constraints: Vec<(&Id, &GenericConstraintDefinition)> = vec!();

        for param in params {
            match param {
                ModelParamDefinition::Generic{id, constraint_type} => {
                    generics.push((id, constraint_type));
                },
                ModelParamDefinition::Metadata{id, type_id, def_value} => {
                    metadata.push((id, type_id, def_value));
                },
                ModelParamDefinition::Constraint{id, constraint} => {
                    constraints.push((id, constraint));
                }
            }
        }

        if !generics.is_empty() {
            self.render(TextToken::Text("<".to_string()));
            for (i, (id, item_type)) in generics.iter().enumerate() {
                self.visit_id(id);
                if let Some(item_type) = item_type {
                    self.render(TextToken::Text(":".to_string()));
                    self.visit_item_type(item_type);
                }

                if i < generics.len() - 1 {
                    self.render(TextToken::Text(",".to_string()));
                }
            }
            self.render(TextToken::Text(">".to_string()));
        }

        if !metadata.is_empty() {
            self.render(TextToken::Text("[".to_string()));
            for (i, (id, item_type, value)) in metadata.iter().enumerate() {
                self.visit_id(id);
                self.render(TextToken::Text(":".to_string()));
                self.visit_item_type(item_type);

                if let Some(value) = value {
                    self.render(TextToken::Text("=".to_string()));
                    self.visit_literal(value);
                }

                if i < metadata.len() - 1 {
                    self.render(TextToken::Text(",".to_string()));
                }
            }
            self.render(TextToken::Text("]".to_string()));
        }
    }

    fn visit_scope(&self, item: &RefScope, is_root: bool) {

        let item = item.borrow();

        match **item {
            Scope::Global(ref items) => {
                self.visit_global(items);
            }
            Scope::Package(ref id, ref items) => {
                self.visit_package(id, items, is_root);
            }
            Scope::Model(ref def) => {
                self.visit_model(def);
            }
            Scope::Error(ref error) => {
                self.visit_error(error);
            }
        }
    }

    fn visit_global(&self, items: &Vec<RefScope>) {
        for item in items {
            self.visit_scope(item, false);
        }
    }

    fn visit_package(&self, id: &Id, items: &Vec<RefScope>, is_root: bool) {
        if is_root {
            self.render(TextToken::Text("package".to_string()));
            self.render(TextToken::Space);
            self.visit_id(id);
            self.render(TextToken::Text(";".to_string()));
            self.render(TextToken::NewLine);

            for item in items {
                self.visit_scope(item, false);
            }

            return;
        }

        self.visit_header_model("package");

        self.visit_id(id);
        self.render(TextToken::Text(" {".to_string()));
        self.render(TextToken::NewLine);

        self.render(TextToken::IncIndent);
        for item in items {
            self.visit_scope(item, false);
        }
        self.render(TextToken::DecIndent);

        self.render(TextToken::Text("}".into()));
        self.render(TextToken::NewLine);
    }

    fn visit_model(&self, def: &ModelDefinition) {

        match def {
            ModelDefinition::Scalar(ref id) => {
                self.visit_scalar(id);
            }
            ModelDefinition::Record(ref id, ref items, ref params) => {
                self.visit_header_model("model");
                self.visit_record_model(id, items, params);
            },
            ModelDefinition::Fragment(ref id, ref items, ref params) => {
                self.visit_header_model("fragment");
                self.visit_record_model(id, items, params);
            }
            ModelDefinition::Enum(ref id, ref items, ref params) => {
                self.visit_header_model("model");
                self.visit_enum_model(id, items, params);
            },
            ModelDefinition::Tuple(ref id, ref items, ref params) => {
                self.visit_header_model("model");
                self.visit_tuple_model(id, items, params);
                self.render(TextToken::NewLine);
            },
            ModelDefinition::Alias(ref id, ref params, ref item_type) => {
                self.visit_header_model("model");
                self.visit_id(id);
                self.visit_model_params_def(params);
                self.render(TextToken::Space);
                self.render(TextToken::Text("= ".to_string()));
                self.visit_item_type(item_type);
                self.render(TextToken::NewLine);
            }
        }
    }

    fn visit_header_model(&self, keyword: &str) {
        self.render(TextToken::LineIndent);
        self.render(TextToken::Text(keyword.to_string()));
        self.render(TextToken::Space);
}

    fn visit_record_model(&self, id: &Id, items: &Vec<RecordItem>, params: &Vec<ModelParamDefinition>) {

            self.visit_id(id);
            self.visit_model_params_def(params);
            self.render(TextToken::Space);
            self.render(TextToken::Text("{".to_string()));
            self.render(TextToken::NewLine);

            self.render(TextToken::IncIndent);
            for item in items {
                self.visit_model_item(item);
            }
            self.render(TextToken::DecIndent);

            self.render(TextToken::Text("}".into()));
            self.render(TextToken::NewLine);
        }

    fn visit_tuple_model(&self, id: &Id, items: &Vec<TupleItem>, params: &Vec<ModelParamDefinition>) {

        self.visit_id(id);
        self.visit_model_params_def(params);
        self.render(TextToken::Text("(".to_string()));

        let mut is_next = false;

        for item in items {

            if is_next {
                self.render(TextToken::Text(", ".to_string()));
            }

            match item {
                TupleItem::Item(ref type_id) => {
                    self.visit_item_type(type_id)
                },
                TupleItem::NamedItem(ref id, ref type_id) => {
                    self.visit_id(id);
                    self.render(TextToken::Text(": ".to_string()));
                    self.visit_item_type(type_id);
                }
            }

            is_next = true;
        }
        self.render(TextToken::Text(")".to_string()));
    }

    fn visit_enum_model(&self, id: &Id, items: &Vec<EnumItem>, params: &Vec<ModelParamDefinition>) {

        self.visit_id(id);
        self.visit_model_params_def(params);
        self.render(TextToken::Space);
        self.render(TextToken::Text("enum".to_string()));
        self.render(TextToken::Space);
        self.render(TextToken::Text("{".to_string()));
        self.render(TextToken::NewLine);

        self.render(TextToken::IncIndent);
        for item in items {
            self.visit_enum_item(item);
        }
        self.render(TextToken::DecIndent);

        self.render(TextToken::Text("}".into()));
        self.render(TextToken::NewLine);
    }

    fn visit_model_item(&self, item: &RecordItem) {
        match item {
            RecordItem::Item(ref id, ref type_id) => {
                self.visit_id(id);
                self.render(TextToken::Text(": ".to_string()));
                self.visit_item_type(type_id);
                self.render(TextToken::NewLine);
            }
            RecordItem::Spread(ref type_id) => {
                self.render(TextToken::Text("... ".to_string()));
                self.visit_item_type(type_id);
                self.render(TextToken::NewLine);
            }
        }
    }

    fn visit_enum_item(&self, item: &EnumItem) {
        match item {
            EnumItem::Item(ref id) => {
                self.visit_id(id);
                self.render(TextToken::NewLine);
            },
            EnumItem::Record(ref id, ref type_id) => {
                self.visit_id(id);
                self.render(TextToken::Space);
                self.visit_item_type(type_id);
            },
            EnumItem::Tuple(ref id, ref type_id) => {
                self.visit_id(id);
                self.visit_item_type(type_id);
                self.render(TextToken::NewLine);
            },
            EnumItem::Enum(ref id, ref type_id) => {
                self.visit_id(id);
                self.render(TextToken::Space);
                self.visit_item_type(type_id);
            }
        }
    }

    fn visit_scalar(&self, id: &Id) {

        self.visit_header_model("scalar");
        self.visit_id(id);
        self.render(TextToken::Text(";".to_string()));
        self.render(TextToken::NewLine);
    }

    fn visit_error(&self, error: &ErrorRecovery<usize, Token, LexicalError>) {
        self.render(TextToken::LineIndent);

        let text = format!("error: {:?}", error).into();
        self.render(TextToken::Text(text));
        self.render(TextToken::NewLine);
    }
}