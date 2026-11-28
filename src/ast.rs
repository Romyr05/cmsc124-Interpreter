use crate::parser::TopLevel;
use crate::token_types::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Bool/Nil are part of the value model
pub enum Value {
    Number(f64),
    Str(String), // owned, so Value needs no lifetime
    Bool(bool),
    Nil,
    // A component literal rendered to an HTML fragment. It is trusted markup:
    // when used as a child it is inserted raw instead of being HTML-escaped.
    Component(String),
    // A script component's raw JavaScript body. It renders as <script>...</script>
    // when used as a child, and can later be used directly as an event handler.
    Script(String),
}

impl Value {
    // Markup values are already generated HTML/JS and bypass text escaping.
    pub fn is_markup(&self) -> bool {
        matches!(self, Value::Component(_) | Value::Script(_))
    }
}

#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub kind: TokenType,
    pub attributes: Vec<Attribute<'a>>,
    pub children: Vec<TopLevel<'a>>,
    // line of the element keyword, used for diagnostics about the element itself
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub kind: TokenType,
    pub value: Expr<'a>,
    // line of the attribute name, used for diagnostics that point at the attribute
    #[allow(dead_code)] // reserved for event-handler type errors
    pub line: usize,
}

// Named document options on render(...): title = "..." and lang = "...".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderOptionKind {
    Title,
    Lang,
}

impl RenderOptionKind {
    pub fn name(self) -> &'static str {
        match self {
            RenderOptionKind::Title => "title",
            RenderOptionKind::Lang => "lang",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderOption<'a> {
    pub kind: RenderOptionKind,
    pub value: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Literal {
        value: Value,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    // a name like x; keeps its line so errors can point at it
    Variable {
        name: String,
        line: usize,
    },
    // an element literal used as a value: let b = button() { "Hi" }
    Element {
        element: Box<Element<'a>>,
    },
}
