use std::cell::RefCell;
use lalrpop_util::ErrorRecovery;
use regex::Regex;
use crate::ast::*;
use crate::lexer::{LexicalError, Token};

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
    fn visit_item_type(&'a self, item_type: &'a ItemType<'a>);
    fn visit_scope(&'a self, item: &'a RefScope, is_root: bool);
    fn visit_global(&'a self, items: &'a Vec<RefScope>);
    fn visit_package(&'a self, id: &'a Id, items: &'a Vec<RefScope>, is_root: bool);
    fn visit_model(&'a self, def: &'a ModelDefinition);
    fn visit_model_item(&'a self, item: &'a RecordItem);
    fn visit_enum_item(&self, item: &EnumItem);
    fn visit_scalar(&'a self, id: &'a Id);
    fn visit_error(&'a self, error: &ErrorRecovery<usize, Token, LexicalError>);
}

pub struct StringRender {
    data: RefCell<Vec<(String, usize)>>,
    current_string: RefCell<String>,
    indent: RefCell<usize>,

    last_token: RefCell<TextToken>,

    new_line_regex: Regex,
    new_line_placeholder: String,
}

impl StringRender {
    pub fn new() -> StringRender {
        StringRender {
            data: RefCell::new(Vec::new()),
            current_string: RefCell::new(String::new()),
            indent: RefCell::new(0),

            last_token: RefCell::new(TextToken::None),

            new_line_regex: Regex::new(r"(?m)(^(?:}|(?:package[\s\w]*;))(\s))?\$nl\$\s").unwrap(),
            new_line_placeholder: String::from("$nl$"),
        }
    }

    pub fn as_string(&self, indent_count: usize) -> String {
        let s= self.data.borrow().iter().map(|(text, level)| {
            let n = level * indent_count;
            let indent = " ".repeat(n.try_into().unwrap_or_default());
            format!("{:}{:}", indent, text)
        }).collect::<Vec<_>>().join("\n");

        self.new_line_regex.replace_all(&s, "$1$2").into()
    }
}

impl Target<TextToken> for StringRender {

    fn render(&self, token: TextToken) {
        match &token {
            TextToken::None => {

            },
            TextToken::Space => {
                if let TextToken::Text(_) = *self.last_token.borrow() {
                    *self.current_string.borrow_mut() += " ";
                }
            },
            TextToken::NewLine => {
                let text = format!("{:}", self.current_string.borrow());
                self.data.borrow_mut().push((text, *self.indent.borrow()));
                self.current_string.borrow_mut().clear();
            },
            TextToken::LineIndent => {
                let text = format!("{:}", self.new_line_placeholder);
                self.data.borrow_mut().push((text, *self.indent.borrow()));
            },
            TextToken::IncIndent => {
                *self.indent.borrow_mut() += 1;
            },
            TextToken::DecIndent => {
                *self.indent.borrow_mut() -= 1;
            },
            TextToken::Text(text) => {
                *self.current_string.borrow_mut() += text;
            },
        }

        *self.last_token.borrow_mut() = token;
    }
}

pub struct ConsoleRender {
    indent_count: usize,
    indent: RefCell<usize>,
    last_token: RefCell<TextToken>,
}

impl ConsoleRender {
    pub fn new(indent_count: usize) -> Self {
        ConsoleRender {
            indent_count,
            indent: RefCell::new(0),
            last_token: RefCell::new(TextToken::None),
        }
    }
}

impl Target<TextToken> for ConsoleRender {

    fn render(&self, token: TextToken) {
        match &token {
            TextToken::None => {

            },
            TextToken::Space => {
                if let TextToken::Text(_) = *self.last_token.borrow() {
                    print!(" ");
                }
            },
            TextToken::NewLine | TextToken::LineIndent => {
                println!();
                let n = *self.indent.borrow() * self.indent_count;
                print!("{:}", " ".repeat(n));
            },
            TextToken::IncIndent => {
                *self.indent.borrow_mut() += 1;
            },
            TextToken::DecIndent => {
                *self.indent.borrow_mut() -= 1;
            },
            TextToken::Text(text) => {
                if let  TextToken::IncIndent | TextToken::DecIndent = *self.last_token.borrow() {
                    let n = *self.indent.borrow() * self.indent_count;
                    print!("\r{:}", " ".repeat(n));
                }

                print!("{}", text);
            },
        }

        *self.last_token.borrow_mut() = token;
    }
}

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

    fn visit_item_type(&self, item_type: &ItemType) {
        match item_type {
            ItemType::Name(ref id) => self.visit_id(id),
            ItemType::Inline(ref model) => {
                match model {
                    ModelDefinition::Fragment(_, _, _) => unreachable!(),
                    ModelDefinition::Record(ref id, ref items, _) => {

                        self.visit_id(id);
                        self.render(TextToken::Space);
                        self.render(TextToken::Text("{".to_string()));
                        self.render(TextToken::NewLine);

                        self.render(TextToken::IncIndent);
                        for item in items {
                            self.visit_model_item(item);
                        }
                        self.render(TextToken::DecIndent);

                        self.render(TextToken::Text("}".to_string()));

                    },
                    ModelDefinition::Tuple(ref id, ref items, _) => {
                        self.visit_id(id);

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
                    },
                    ModelDefinition::Enum(ref id, ref items, _) => {

                        self.render(TextToken::Text("enum".to_string()));
                        self.render(TextToken::Space);
                        self.visit_id(id);
                        self.render(TextToken::Space);
                        self.render(TextToken::Text("{".to_string()));
                        self.render(TextToken::NewLine);

                        self.render(TextToken::IncIndent);
                        for item in items {
                            self.visit_enum_item(item);
                        }
                        self.render(TextToken::DecIndent);

                        self.render(TextToken::Text("}".to_string()));
                    },
                    ModelDefinition::Scalar(_) => unreachable!(),
                }
            },
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

        self.render(TextToken::LineIndent);

        self.render(TextToken::Text("package".to_string()));
        self.render(TextToken::Space);
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
            ModelDefinition::Record(ref id, ref items, ref _params) => {
                self.render(TextToken::LineIndent);

                self.render(TextToken::Text("model".to_string()));
                self.render(TextToken::Space);
                self.visit_id(id);
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
            },
            ModelDefinition::Fragment(ref id, ref items, ref _params) => {
                self.render(TextToken::LineIndent);

                self.render(TextToken::Text("fragment".to_string()));
                self.render(TextToken::Space);
                self.visit_id(id);
                self.render(TextToken::Text(" {".to_string()));
                self.render(TextToken::NewLine);

                self.render(TextToken::IncIndent);
                for item in items {
                    self.visit_model_item(item);

                }
                self.render(TextToken::DecIndent);

                self.render(TextToken::Text("}".into()));
                self.render(TextToken::NewLine);
            }
            ModelDefinition::Enum(ref id, ref items, ref _params) => {
                self.render(TextToken::LineIndent);

                self.render(TextToken::Text("enum".to_string()));
                self.render(TextToken::Space);
                self.visit_id(id);
                self.render(TextToken::Space);
                self.render(TextToken::Text("{".to_string()));
                self.render(TextToken::NewLine);

                self.render(TextToken::IncIndent);
                for item in items {

                    match item {
                        EnumItem::Item(ref id) => {
                            self.visit_id(id);
                            self.render(TextToken::NewLine);
                        },
                        EnumItem::Record(ref id, ref type_id) => {
                            self.visit_id(id);
                            self.render(TextToken::Space);
                            self.visit_item_type(type_id);
                            self.render(TextToken::NewLine);
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
                            self.render(TextToken::NewLine);
                        }
                    }
                }
                self.render(TextToken::DecIndent);

                self.render(TextToken::Text("}".into()));
                self.render(TextToken::NewLine);
            },
            ModelDefinition::Tuple(ref id, ref items, ref _params) => {
                self.render(TextToken::LineIndent);

                self.render(TextToken::Text("model".to_string()));
                self.render(TextToken::Space);
                self.visit_id(id);
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

                self.render(TextToken::NewLine);
            }
        }
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
            },
            EnumItem::Enum(ref id, ref type_id) => {
                self.visit_id(id);
                self.render(TextToken::Text(": enum".to_string()));
                self.render(TextToken::Space);
                self.visit_item_type(type_id);
            }
        }
    }

    fn visit_scalar(&self, id: &Id) {
        self.render(TextToken::LineIndent);

        self.render(TextToken::Text("scalar".to_string()));
        self.render(TextToken::Space);
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