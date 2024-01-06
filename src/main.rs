use rustc_hash::FxHashMap;
use std::env;
use std::fs::File;
use std::io::{stdin, Read};

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

    DecrementLoop(u8),
    IncrementLoop(u8),

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
                        let loop_start = loop_stack.pop().unwrap(); // Index it should jump to in order to restart the loop
                        let loop_end = instructions.len(); // Index it should jump to in order to skip the loop

                        if loop_end - loop_start - 1 == 1 {
                            // Only one type of instruction there
                            let looped_instruction = &instructions[loop_start + 1];
                            match looped_instruction {
                                Instruction::Decrement(decrement) => {
                                    instructions[loop_start] =
                                        Instruction::DecrementLoop(*decrement);

                                    instructions.remove(loop_start + 1);
                                }
                                Instruction::Increment(increment) => {
                                    instructions[loop_start] =
                                        Instruction::IncrementLoop(*increment);
                                    instructions.remove(loop_start + 1);
                                }
                                _ => {
                                    instructions.push(Instruction::LoopEnd(loop_start));
                                    instructions[loop_start] = Instruction::LoopStart(loop_end);
                                }
                            }
                        } else {
                            instructions.push(Instruction::LoopEnd(loop_start));
                            instructions[loop_start] = Instruction::LoopStart(loop_end);
                        }
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
                instruction_index += 1
            }
            Instruction::Increment(increment) => {
                memory
                    .entry(pointer)
                    .and_modify(|cell| *cell += increment)
                    .or_insert(*increment);
                instruction_index += 1
            }
            Instruction::Decrement(decrement) => {
                memory
                    .entry(pointer)
                    .and_modify(|cell| *cell -= decrement)
                    .or_insert(0 - *decrement);
                instruction_index += 1
            }
            Instruction::DecrementLoop(decrement) => {
                memory.entry(pointer).and_modify(|cell| {
                    if *cell % *decrement == 0 {
                        *cell = 0
                    } else {
                        panic!("Infinite loop detected")
                    }
                });
                instruction_index += 1
            }
            Instruction::IncrementLoop(increment) => {
                memory.entry(pointer).and_modify(|cell| {
                    if *cell % *increment == 0 {
                        *cell = 0
                    } else {
                        panic!("Infinite loop detected")
                    }
                });
                instruction_index += 1
            }
            Instruction::LoopStart(loop_end) => {
                let cell = *memory.get(&pointer).unwrap_or(&0);
                if cell == 0 {
                    instruction_index = *loop_end
                } else {
                    instruction_index += 1
                }
            }
            Instruction::LoopEnd(loop_start) => {
                let cell = *memory.get(&pointer).unwrap_or(&0);
                if cell != 0 {
                    instruction_index = *loop_start
                } else {
                    instruction_index += 1
                }
            }
            Instruction::Output => {
                print!("{}", *memory.get(&pointer).unwrap_or(&0) as char);

                instruction_index += 1
            }
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                stdin().read_exact(&mut input).unwrap();
                memory.insert(pointer, input[0]);

                instruction_index += 1
            }
        }
    }

    memory
}

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

    execute(compiled);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
