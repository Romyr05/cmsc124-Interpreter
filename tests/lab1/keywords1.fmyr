mod scanner;
mod token_app;
use crate::scanner::Tokenizer;
use crate::token_app::Token;

fn main() {
    //test works, ignore the compiler warnings
    let source = "let x = 42 + 5 button () == aa";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<Token> = tokenizer.collect();
    for token in tokens {
        println!("{:?}", token);
    }
}
