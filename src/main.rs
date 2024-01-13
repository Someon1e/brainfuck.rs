use std::fs;
use std::io::{stdin, stdout, StdoutLock, Write};

mod compile;
use compile::Compiler;

mod lexer;
use lexer::Lexer;

mod interpreter;
use interpreter::execute;

mod into_rust;
use into_rust::to_rust;

fn ask(question: &str, stdout: &mut StdoutLock) -> String {
    write!(stdout, "{question}").unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    return input.trim().to_owned();
}

fn main() {
    let input;
    let option;
    {
        let mut stdout = stdout().lock();

        let input_type = ask("(A) File directory or (B) text input? ", &mut stdout);
        if input_type == "A" {
            input = fs::read_to_string(ask("File directory: ", &mut stdout)).unwrap()
        } else if input_type == "B" {
            input = ask("Code: ", &mut stdout)
        } else {
            panic!("Invalid input")
        }

        option = ask("(A) Interpret or (B) transpile into rust? ", &mut stdout);
        if option != "A" {
            assert_eq!(option, "B", "Invalid input")
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
