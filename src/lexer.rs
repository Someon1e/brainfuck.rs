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

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {chars: input.chars()}
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let character = self.chars.next();
        if let Some(character) = character {
            Some(match character {
                '+' => Token::Increment,
                '-' => Token::Decrement,

                '>' => Token::Forward,
                '<' => Token::Backward,

                '[' => Token::LoopStart,
                ']' => Token::LoopEnd,

                '.' => Token::Output,
                ',' => Token::Input,
                _ => Token::Comment,
            })
        } else {
            None
        }
    }
}