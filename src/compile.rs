use crate::lexer::Token;

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
pub fn compile(tokens: &Vec<Token>) -> Vec<Instruction> {
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
                                Instruction::Move(offset) => {
                                    instructions[loop_start] = Instruction::MoveLoop(*offset);
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
