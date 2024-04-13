#[derive(Debug, PartialEq, Eq)]
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

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mapped = input.chars().map(|character| match character {
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

    mapped
}
