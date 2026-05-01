use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
enum TokenType {
    #[token("\n")]
    NewLine,
    
    #[token("#")]
    Hash,
    
    #[token(":")]
    Colon,
    
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    
    // LITERALS
    #[regex(r"[0-9]+")]
    IntLit,

    #[regex(r"[+-]?(?:\d+\.\d*|\.\d+)(?:[eE][+-]?\d+)?|\d+[eE][+-]?\d+")]
    FloatLit,

    #[regex(r"[+-]?\d+g")] // int with suffix `g`
    GroupLit,

    // OPS
    #[token("spawn")]
    Spawn,

    #[token("alpha")]
    Alpha,
}

fn lex(source: &String) {
    let mut lex = TokenType::lexer(source);
    
    while let Some(token) = lex.next() {
        println!("{token:?}")
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::lexer::lex;

    #[test]
    fn lex_test() {
        let source = fs::read_to_string("examples/lexer.ln").unwrap();
        lex(&source);
    }
}