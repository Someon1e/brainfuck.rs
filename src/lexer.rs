#[derive(Debug, PartialEq)]
pub enum Token {
    Forward,
    Backward,
    Increment,
    Decrement,

    LoopStart,
    LoopEnd,

    Output,
    Input,

    Comment,
}

pub fn lex(input: &String) -> Vec<Token> {
    let mut tokens = vec![];
    for character in input.chars() {
        tokens.push(match character {
            '>' => Token::Forward,
            '<' => Token::Backward,

            '+' => Token::Increment,
            '-' => Token::Decrement,

            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,

            '.' => Token::Output,
            ',' => Token::Input,
            _ => Token::Comment,
        });
    }
    return tokens;
}
