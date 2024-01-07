#[derive(Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,

    Forward,
    Backward,

    LoopStart,
    LoopEnd,

    Output,
    Input,

    Comment,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(input.len());
    for character in input.chars() {
        tokens.push(match character {
            '+' => Token::Increment,
            '-' => Token::Decrement,

            '>' => Token::Forward,
            '<' => Token::Backward,

            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,

            '.' => Token::Output,
            ',' => Token::Input,
            _ => Token::Comment,
        });
    }
    tokens
}
