#[derive(Debug)]
#[allow(dead_code)]  //Remove this once we have parser, Warnings happen bcos of fields never read (lifetime) 
                        // and Words not constructed
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
    IdName(&'a str),
    ClassName(&'a str),
    //styling
    StyleAlign(&'a str),
    StylePad(&'a str),
    StyleMargin(&'a str),
    StyleHeight(&'a str),
    StyleWidth(&'a str),
    StyleColor(&'a str),
    StyleBorder(&'a str),
}