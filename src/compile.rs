use crate::lexer::{Token, Lexer};

#[derive(Debug)]
pub enum Instruction {
    Move(isize),

    Increment(u8),
    Decrement(u8),

    DecrementLoop(u8),
    IncrementLoop(u8),
    MoveLoop(isize),

    LoopStart(usize),
    LoopEnd(usize),

    Output,
    Input,
}

#[derive(Debug, PartialEq)]
enum CompilingInstruction {
    Move,
    Increment,

    None,
}

fn push_compiling_instruction(
    instructions: &mut Vec<Instruction>,
    compiling_instruction: &mut CompilingInstruction,
    value: &mut isize,
) {
    if *compiling_instruction != CompilingInstruction::None {
        instructions.push(match compiling_instruction {
            CompilingInstruction::Move => Instruction::Move(*value),
            CompilingInstruction::Increment => {
                if value.is_positive() {
                    Instruction::Increment(*value as u8)
                } else {
                    Instruction::Decrement(value.abs() as u8)
                }
            }
            CompilingInstruction::None => unreachable!(),
        });
        *compiling_instruction = CompilingInstruction::None;
        *value = 0
    }
}
pub fn compile(tokens: Lexer) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    let mut value: isize = 0;
    let mut compiling_instruction: CompilingInstruction = CompilingInstruction::None;

    let mut loop_stack = vec![];

    for token in tokens {
        match token {
            Token::Forward | Token::Backward => {
                if compiling_instruction != CompilingInstruction::Move {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = CompilingInstruction::Move;
                }
                value += if let Token::Forward = token { 1 } else { -1 };
                continue;
            }
            Token::Increment | Token::Decrement => {
                if compiling_instruction != CompilingInstruction::Increment {
                    push_compiling_instruction(
                        &mut instructions,
                        &mut compiling_instruction,
                        &mut value,
                    );
                    compiling_instruction = CompilingInstruction::Increment;
                }
                value += if let Token::Increment = token { 1 } else { -1 };
                continue;
            }
            Token::Comment => continue,
            _ => {}
        }

        push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);
        match token {
            Token::LoopStart => {
                loop_stack.push(instructions.len());
                instructions.push(Instruction::LoopStart(0)) // temp 0
            }
            Token::LoopEnd => {
                let loop_start = loop_stack.pop().expect("loop end without start"); // Index it should jump to in order to restart the loop
                let loop_end = instructions.len(); // Index it should jump to in order to skip the loop

                let replacement = if loop_end - loop_start - 1 == 1 {
                    // Only one type of instruction there
                    match instructions[loop_start + 1] {
                        Instruction::Decrement(decrement) => {
                            instructions.remove(loop_start + 1);

                            Instruction::DecrementLoop(decrement)
                        }
                        Instruction::Increment(increment) => {
                            instructions.remove(loop_start + 1);

                            Instruction::IncrementLoop(increment)
                        }
                        Instruction::Move(offset) => {
                            instructions.remove(loop_start + 1);

                            Instruction::MoveLoop(offset)
                        }
                        _ => {
                            instructions.push(Instruction::LoopEnd(loop_start));

                            Instruction::LoopStart(loop_end)
                        }
                    }
                } else {
                    instructions.push(Instruction::LoopEnd(loop_start));
                    Instruction::LoopStart(loop_end)
                };
                instructions[loop_start] = replacement;
            }
            Token::Input => instructions.push(Instruction::Input),
            Token::Output => instructions.push(Instruction::Output),
            _ => unreachable!(),
        }
    }

    push_compiling_instruction(&mut instructions, &mut compiling_instruction, &mut value);

    if !loop_stack.is_empty() {
        panic!("Unclosed loop");
    }

    instructions
}
