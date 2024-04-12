#![deny(clippy::all)]

use std::fs;
use std::io::{stdin, stdout, BufRead, Write};

mod compile;
use compile::Compiler;

mod lexer;
use lexer::Lexer;

mod interpreter;
use interpreter::execute;

mod into_rust;
use into_rust::to_rust;

/// Initial memory capacity, however memory will resize when full.
pub const INITIAL_MEMORY_CAPACITY: usize = 32;

/// When out of memory, increase size by this many bytes.
pub const MEMORY_RESIZE_AMOUNT: usize = 16;

fn main() {
    let input;
    let option;
    {
        let mut stdout = stdout().lock();
        let mut stdin = stdin().lock();

        macro_rules! ask {
            ($question:expr) => {{
                write!(stdout, "{}", $question).unwrap();
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
    let lexed = Lexer::new(&input);

    //println!("{:#?}", lexed);

    let mut compiler = Compiler::new(lexed);
    let compiled = compiler.compile();
    //println!("{:#?}", compiled);

    if option == "A" {
        execute(compiled);
    } else {
        fs::write("output.rs", to_rust(compiled)).unwrap();
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}
