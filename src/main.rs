#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::fs;
use std::io::{stdin, stdout, BufRead, Write};

use brainfuck::compile::Compiler;
use brainfuck::interpreter::execute;
use brainfuck::into_rust::to_rust;
use brainfuck::lexer::lex;

fn main() {
    let input;
    let option;
    {
        let mut stdout = stdout().lock();
        let mut stdin = stdin().lock();

        macro_rules! ask {
            ($question:expr) => {{
                write!(stdout, "\x1b[37m{}\x1b[0m", $question).unwrap();
                stdout.flush().unwrap();

                let mut input = String::new();
                stdin.read_line(&mut input).unwrap();
                input.trim().to_string()
            }};
        }

        let input_type = ask!("(A) File directory or (B) text input? ");
        if input_type == "A" {
            input = fs::read_to_string(ask!("File directory: ")).unwrap();
        } else if input_type == "B" {
            input = ask!("Code: ");
        } else {
            panic!("Invalid input")
        }

        option = ask!("(A) Interpret or (B) transpile into rust? ");
        if option != "A" {
            assert_eq!(option, "B", "Invalid input");
        };
    }

    let before = std::time::Instant::now();
    let lexed = lex(&input);

    //println!("{:?}", lex(&input).collect::<Vec<crate::lexer::Token>>());

    let mut compiler = Compiler::new(lexed);
    let compiled = compiler.compile();
    //println!("{compiled:?}");

    if option == "A" {
        execute(compiled);
    } else {
        fs::write("output.rs", to_rust(compiled)).unwrap();
    }

    let mut stdout = stdout().lock();
    stdout.flush().unwrap();
    write!(
        stdout,
        "\x1b[37mElapsed time: {:.2?}\x1b[0m",
        before.elapsed()
    )
    .unwrap();
}
