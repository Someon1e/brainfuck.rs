use fxhash::FxHashMap;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum Token {
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

            '.' => Token::Output,
            ',' => Token::Input,
            _ => Token::Comment,
        });
    }
    return tokens;
}

#[derive(Debug)]
enum Instruction {
    Move(isize),

    Increment(u8),
    Decrement(u8),

    LoopStart(usize),
    LoopEnd(usize),

    Output,
    Input,
}

#[derive(Debug, PartialEq)]
enum CompilingInstruction {
    Move,
    Increment,
    Decrement,
}

fn push_compiling_instruction(
    instructions: &mut Vec<Instruction>,
    compiling_instruction: &mut Option<&CompilingInstruction>,
    value: &mut isize,
) {
    if let Some(unwrapped_compiling_instruction) = compiling_instruction {
        instructions.push(match unwrapped_compiling_instruction {
            CompilingInstruction::Move => Instruction::Move(*value),
            CompilingInstruction::Increment => Instruction::Increment(*value as u8),
            CompilingInstruction::Decrement => Instruction::Decrement(*value as u8),
        });
        *compiling_instruction = None;
        *value = 0
    }
}
fn compile(tokens: &Vec<Token>) -> Vec<Instruction> {
    // Merge increment and decrement instruction
    let mut instructions: Vec<Instruction> = vec![];

    let mut value: isize = 0;
    let mut compiling_instruction: Option<&CompilingInstruction> = None;

    let mut loop_stack = vec![];

    for token in tokens {
        match token {
            Token::Forward => {
                if compiling_instruction != Some(&CompilingInstruction::Move) {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = Some(&CompilingInstruction::Move);
                }
                value += 1
            }
            Token::Backward => {
                if compiling_instruction != Some(&CompilingInstruction::Move) {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = Some(&CompilingInstruction::Move);
                }
                value -= 1
            }
            Token::Increment => {
                if compiling_instruction != Some(&CompilingInstruction::Increment) {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = Some(&CompilingInstruction::Increment);
                }
                value += 1
            }
            Token::Decrement => {
                if compiling_instruction != Some(&CompilingInstruction::Decrement) {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = Some(&CompilingInstruction::Decrement);
                }
                value += 1
            }
            _ => {
                push_compiling_instruction(
                    &mut instructions,
                    &mut compiling_instruction,
                    &mut value,
                );
                match token {
                    Token::LoopStart => {
                        loop_stack.push(instructions.len());
                        instructions.push(Instruction::LoopStart(0)) // temp 0
                    }
                    Token::LoopEnd => {
                        let loop_start = loop_stack.pop().unwrap();
                        instructions.push(Instruction::LoopEnd(loop_start));
                        instructions[loop_start] = Instruction::LoopStart(instructions.len())
                    }
                    Token::Input => instructions.push(Instruction::Input),
                    Token::Output => instructions.push(Instruction::Output),
                    Token::Comment => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    if !loop_stack.is_empty() {
        panic!("Unclosed loop");
    }

    instructions
}

fn execute(instructions: Vec<Instruction>) -> FxHashMap<isize, u8> {
    let mut memory: FxHashMap<isize, u8> = FxHashMap::default();
    let mut pointer: isize = 0;

    let mut instruction_index = 0;
    while instruction_index != instructions.len() {
        let instruction = &instructions[instruction_index];
        match instruction {
            Instruction::Move(offset) => {
                pointer += offset;
            }
            Instruction::Increment(increment) => {
                let cell = memory.entry(pointer).or_insert(0);
                *cell += increment;
            }
            Instruction::Decrement(decrement) => {
                let cell = memory.entry(pointer).or_insert(0);
                *cell -= decrement;
            }
            Instruction::LoopStart(loop_end) => {
                let cell = *memory.get(&pointer).unwrap_or(&0);
                if cell == 0 {
                    instruction_index = *loop_end
                }
            }
            Instruction::LoopEnd(loop_start) => {
                let cell = *memory.get(&pointer).unwrap_or(&0);
                if cell != 0 {
                    instruction_index = *loop_start
                }
            }
            Instruction::Output => {
                println!("{}", *memory.get(&pointer).unwrap_or(&0) as char)
            }
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin().read_exact(&mut input).unwrap();
                memory.insert(pointer, input[0]);
            }
        }
        instruction_index += 1
    }

    memory
}

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let lexed = lex(&input);
    println!("{:#?}", lexed);

    let compiled = compile(&lexed);
    println!("{:#?}", compiled);

    execute(compiled);
}
