use std::{env, fs};
use std::fs::File;
use std::io::{stdin, Read};

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

    let mut input = String::new();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        stdin.read_line(&mut input).unwrap();
    } else {
        let mut file = File::open(&args[1]).unwrap();
        file.read_to_string(&mut input).unwrap();
    }

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
