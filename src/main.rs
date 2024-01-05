use std::io;

#[derive(Debug, PartialEq)]
enum Token {
    Forward,
    Backward,
    Increment,
    Decrement,

    LoopStart,
    LoopEnd,

    Input,
    Output,

    Comment,
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
            _ => Token::Comment,
        });
    }
    return tokens;
}

#[derive(Debug)]
enum Instruction {
    Move(i32),

    Increment(i32),

    LoopStart,
    LoopEnd,

    Input,
    Output,
}

#[derive(Debug, PartialEq)]
enum CompilingInstruction {
    Move,
    Increment,
}

fn push_compiling_instruction(
    instructions: &mut Vec<Instruction>,
    compiling_instruction: &mut Option<&CompilingInstruction>,
    value: &mut i32) {
    if let Some(unwrapped_compiling_instruction) = compiling_instruction {
        instructions.push(match unwrapped_compiling_instruction {
            CompilingInstruction::Move => Instruction::Move(*value),
            CompilingInstruction::Increment => Instruction::Increment(*value)
        });
        *compiling_instruction = None;
        *value = 0
    }
}
fn compile(tokens: &Vec<Token>) -> Vec<Instruction> {
    // Merge increment and decrement instruction
    let mut instructions: Vec<Instruction> = vec![];

    let mut value: i32 = 0;
    let mut compiling_instruction: Option<&CompilingInstruction> = None;

    for token in tokens {
        match token {
            Token::Forward => {
                if compiling_instruction != Some(&CompilingInstruction::Move) {
                    push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
                    compiling_instruction = Some(&CompilingInstruction::Move);
                }
                value += 1
            }
            Token::Backward => {
                if compiling_instruction != Some(&CompilingInstruction::Move) {
                    push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
                    compiling_instruction = Some(&CompilingInstruction::Move);
                }
                value -= 1
            }
            Token::Increment => {
                if compiling_instruction != Some(&CompilingInstruction::Increment) {
                    push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
                    compiling_instruction = Some(&CompilingInstruction::Increment);
                }
                value += 1
            }
            Token::Decrement => {
                if compiling_instruction != Some(&CompilingInstruction::Increment) {
                    push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
                    compiling_instruction = Some(&CompilingInstruction::Increment);
                }
                value -= 1
            }
            _ => {
                push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
                match token {
                    Token::LoopStart => instructions.push(Instruction::LoopStart),
                    Token::LoopEnd => instructions.push(Instruction::LoopEnd),
                    Token::Input => instructions.push(Instruction::Input),
                    Token::Output => instructions.push(Instruction::Output),
                    Token::Comment => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    instructions
}

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let lexed = lex(&input);
    println!("{:#?}", lexed);

    let compiled = compile(&lexed);
    println!("{:#?}", compiled);
}
