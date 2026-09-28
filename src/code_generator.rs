use crate::ast::{Expr};

// boilerplate components
#[expect(dead_code)]
const HTML_START: &str = "<!DOCTYPE html><html lang\"html\">";
const HTML_END: &str = "</html>";
const HEAD_START: &str = "<head>";
const HEAD_END: &str = "</head>";
const BODY_START: &str = "<body>";
const BODY_END: &str = "</body>";

pub struct CodeGenerator<'a> {
    ast: Expr<'a>,
    code: String,
}

// for now, generate empty html boilerplate
impl<'a> CodeGenerator<'a> {
    fn load(&mut self, ast: Expr<'a>) {
        self.ast = ast;
    }

    fn generate(&mut self){
        self.code = format!("{}{}{}{}{}{}", HTML_START, HEAD_START, HEAD_END, BODY_START, BODY_END, HTML_END);
    }
}