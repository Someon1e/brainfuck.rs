use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Instruction {
    Forward(usize),
    Backward(usize),

    Increment(u8),
    Decrement(u8),

    SetZero,
    IncrementLoop(u8),

    MultiplyForward(usize, u8),

    ForwardLoop(usize),
    BackwardLoop(usize),

    LoopStart(usize),
    LoopEnd(usize),

    Output,
    Input,

    Stop,
}

#[derive(Debug, PartialEq)]
enum CompilingInstruction {
    Move,
    Increment,

    None,
}

pub struct Compiler<'a> {
    tokens: Lexer<'a>,
    instructions: Vec<Instruction>,
    value: isize,
    loop_stack: Vec<usize>,
    compiling_instruction: CompilingInstruction,
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: Lexer<'a>) -> Self {
        Self {
            tokens,
            instructions: vec![],
            loop_stack: vec![],
            compiling_instruction: CompilingInstruction::None,
            value: 0,
        }
    }
    fn compile_compiling_instruction(&mut self) {
        if self.compiling_instruction == CompilingInstruction::None {
            return;
        }
        self.instructions.push(match self.compiling_instruction {
            CompilingInstruction::Move => {
                if self.value.is_positive() {
                    Instruction::Forward(self.value as usize)
                } else {
                    Instruction::Backward(self.value.unsigned_abs())
                }
            }
            CompilingInstruction::Increment => {
                if self.value.is_positive() {
                    Instruction::Increment(self.value as u8)
                } else {
                    Instruction::Decrement(self.value.unsigned_abs() as u8)
                }
            }
            CompilingInstruction::None => unreachable!(),
        });
        self.compiling_instruction = CompilingInstruction::None;
        self.value = 0;
    }
    fn forward_backward(&mut self, token: &Token) {
        if self.compiling_instruction != CompilingInstruction::Move {
            self.compile_compiling_instruction();
            self.compiling_instruction = CompilingInstruction::Move;
        }
        self.value += if matches!(token, Token::Forward) {
            1
        } else {
            -1
        };
    }
    fn increment_decrement(&mut self, token: &Token) {
        if self.compiling_instruction != CompilingInstruction::Increment {
            self.compile_compiling_instruction();
            self.compiling_instruction = CompilingInstruction::Increment;
        }
        self.value += if matches!(token, Token::Increment) {
            1
        } else {
            -1
        };
    }
    fn start_loop(&mut self) {
        self.loop_stack.push(self.instructions.len());
        self.instructions.push(Instruction::LoopStart(0)); // temp 0
    }
    fn end_loop(&mut self) {
        let loop_start = self.loop_stack.pop().expect("loop end without start"); // Index of loop start instruction
        let loop_end = self.instructions.len(); // Index of loop end instruction

        if loop_end - loop_start - 1 == 0 {
            return;
        }
        if loop_end - loop_start - 1 == 1 {
            // Only one type of instruction there
            self.instructions[loop_start] = match *self.instructions.get(loop_start + 1).unwrap() {
                Instruction::Decrement(value) | Instruction::Increment(value) => {
                    self.instructions.remove(loop_start + 1);

                    if value == 1 {
                        Instruction::SetZero
                    } else {
                        Instruction::IncrementLoop(value)
                    }
                }
                Instruction::Forward(offset) => {
                    self.instructions.remove(loop_start + 1);

                    Instruction::ForwardLoop(offset)
                }
                Instruction::Backward(offset) => {
                    self.instructions.remove(loop_start + 1);

                    Instruction::BackwardLoop(offset)
                }
                _ => {
                    self.instructions.push(Instruction::LoopEnd(loop_start + 1));
                    Instruction::LoopStart(loop_end + 1)
                }
            }
        } else {
            let mut multipliers = None;

            if matches!(
                *self.instructions.get(loop_start + 1).unwrap(),
                Instruction::Decrement(_)
            ) {
                let mut compiling_multiplier = 0;
                let mut compiling_multipliers = Vec::with_capacity(2);
                let mut total_offset = 0;
                for index in loop_start + 2..loop_end {
                    // Iterate all instructions inside the loop, except the first
                    let inner = self.instructions.get(index).unwrap();
                    match inner {
                        Instruction::Forward(offset) => {
                            if compiling_multiplier != 0 {
                                compiling_multipliers.push((total_offset, compiling_multiplier));
                                compiling_multiplier = 0;
                            }
                            total_offset += offset;
                        }
                        Instruction::Backward(offset) => {
                            if compiling_multiplier != 0 {
                                compiling_multipliers.push((total_offset, compiling_multiplier));

                                #[allow(unused_assignments)]
                                {
                                    compiling_multiplier = 0;
                                }
                            }

                            if index == loop_end - 1 // This should be the last instruction
                                && total_offset == *offset // and should undo all the offsets
                                && compiling_multipliers.len() != 0
                            // and there should be multipliers
                            {
                                multipliers = Some(compiling_multipliers);
                            }
                            break;
                        }
                        Instruction::Increment(increment) => compiling_multiplier += increment,
                        _ => break,
                    }
                }
            }

            if let Some(multipliers) = multipliers {
                self.instructions.truncate(loop_start);
                for (offset, multiplier) in multipliers.iter() {
                    self.instructions
                        .push(Instruction::MultiplyForward(*offset, *multiplier))
                }
                self.instructions.push(Instruction::SetZero)
            } else {
                self.instructions.push(Instruction::LoopEnd(loop_start + 1));
                self.instructions[loop_start] = Instruction::LoopStart(loop_end + 1)
            }
        };
    }

    pub fn compile(&mut self) -> &Vec<Instruction> {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Increment | Token::Decrement => self.increment_decrement(&token),
                Token::Forward | Token::Backward => self.forward_backward(&token),

                Token::LoopStart => {
                    self.compile_compiling_instruction();
                    self.start_loop();
                }
                Token::LoopEnd => {
                    self.compile_compiling_instruction();
                    self.end_loop();
                }
                Token::Input => {
                    self.compile_compiling_instruction();
                    self.instructions.push(Instruction::Input);
                }
                Token::Output => {
                    self.compile_compiling_instruction();
                    self.instructions.push(Instruction::Output);
                }

                Token::Comment => {}
            }
        }

        self.compile_compiling_instruction();
        self.instructions.push(Instruction::Stop);

        assert!(self.loop_stack.is_empty(), "Unclosed loop");

        &self.instructions
    }
}
