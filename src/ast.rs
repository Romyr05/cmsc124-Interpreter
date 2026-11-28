use crate::parser::TopLevel;
use crate::token_types::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Bool/Nil are part of the value model
pub enum Value {
    Number(f64),
    Str(String), // owned, so Value needs no lifetime
    Bool(bool),
    Nil,
    // A component literal rendered to markup. It is trusted: when used as a
    // child its parts are inserted raw instead of being HTML-escaped.
    Component(Markup),
    // A script component's raw JavaScript body. It renders as <script>...</script>
    // when used as a child, and can later be used directly as an event handler.
    Script(String),
}

impl Value {
    // Markup values are already generated HTML/JS and bypass text escaping.
    pub fn is_markup(&self) -> bool {
        matches!(self, Value::Component(_) | Value::Script(_))
    }

    // The markup this value contributes when inserted into a page. A script
    // value is JavaScript, so it goes to the end of <body> like every script.
    pub fn as_markup(&self) -> Option<Markup> {
        match self {
            Value::Component(markup) => Some(markup.clone()),
            Value::Script(javascript) => Some(Markup {
                end: script_tag(javascript),
                ..Markup::default()
            }),
            _ => None,
        }
    }
}

// Generated output split by where it belongs in the document. CSS is collected
// into <head>, JavaScript into the end of <body>, and everything else stays in
// source order in `body`. A component keeps all three, so a style or script
// inside a bound component still lands in the right place when it is reused.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Markup {
    pub head: String,
    pub body: String,
    pub end: String,
}

impl Markup {
    // Add another fragment's parts after this one's, part by part.
    pub fn append(&mut self, other: &Markup) {
        self.head.push_str(&other.head);
        self.body.push_str(&other.body);
        self.end.push_str(&other.end);
    }

    // All parts as one string, in document order (print/--eval output).
    pub fn to_html(&self) -> String {
        format!("{}{}{}", self.head, self.body, self.end)
    }
}

pub fn script_tag(javascript: &str) -> String {
    format!("<script>{}</script>", javascript)
}

pub fn style_tag(css: &str) -> String {
    format!("<style>{}</style>", css)
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
