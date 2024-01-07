use std::io::stdin;
use std::{env, fs};

mod compile;
use compile::compile;

mod lexer;
use lexer::lex;

mod interpreter;
use interpreter::execute;

mod into_rust;
use into_rust::to_rust;

fn main() {
    let stdin = stdin();
    let args: Vec<String> = env::args().collect();

    let input = if args.len() == 1 {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input
    } else {
        fs::read_to_string(&args[1]).unwrap()
    };

    let before = std::time::Instant::now();
    let lexed = lex(&input);
    //println!("{:#?}", lexed);

    let compiled = compile(&lexed);
    //println!("{:#?}", compiled);

    if false {
        execute(compiled);
    } else {
        fs::write("output.rs", to_rust(compiled)).unwrap();
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}
