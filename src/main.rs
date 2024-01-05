use std::io;

#[derive(Debug)]
enum Token {
    Backward, Forward,
    Increment, Decrement,

    LoopStart, LoopEnd,

    Input, Output,

    Comment
}

fn lex(input: &String) -> Vec<Token> {
    let mut tokens = vec![];
    for character in input.chars() {
        tokens.push(match character {
            '>' => Token::Forward,
            '<' => Token::Backward,
            '+' => Token::Increment,
            '-' => Token::Decrement,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            '.' => Token::Input,
            ',' => Token::Output,
            _ => Token::Comment
        });
    }
    return tokens
}

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    println!("{:#?}", lex(&input));
}
