use std::cell::RefCell;
use crate::transform::{Target, TextToken};

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