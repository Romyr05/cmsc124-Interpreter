#[derive(Debug)]
pub enum Token<'a> {
    LeftParen(&'a str),
    RightParen(&'a str),
    Plus(&'a str),
    Equal(&'a str),
    Number(&'a str),
    Word(&'a str),
    WhiteSpace(&'a str),
    Identifier(&'a str),
    // keyword tokens below
    Button(&'a str),
    Box(&'a str),
    Text(&'a str),
    Image(&'a str),
    Div(&'a str),
    Paragraph(&'a str),
    // attributes
    ID_name(&'a str),
    Class_name(&'a str),
    //styling
    Style_Align(&'a str),
    Style_Pad(&'a str),
    Style_Margin(&'a str),
    Style_Height(&'a str),
    Style_Width(&'a str),
    Style_Color(&'a str),
    Style_Border(&'a str),
}