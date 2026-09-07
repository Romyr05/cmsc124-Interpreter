mod scanner;
use crate::scanner::Tokenizer;
use crate::scanner::Token;

fn main() {
    //test works, ignore the compiler warnings
    let source = "let x = 42 + 5";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<Token> = tokenizer.collect();
    for token in tokens {
        println!("{:?}", token);
    }
}
