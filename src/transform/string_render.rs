use std::cell::RefCell;
use regex::Regex;
use crate::transform::{Target, TextToken};

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
                if text.len() > 0 {
                    self.data.borrow_mut().push((text, *self.indent.borrow()));
                    self.current_string.borrow_mut().clear();
                }
            },
            TextToken::LineIndent => {
                let text = format!("{:}", self.new_line_placeholder);
                self.data.borrow_mut().push((text, 0));
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