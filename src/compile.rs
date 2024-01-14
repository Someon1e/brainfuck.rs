use crate::lexer::{Lexer, Token};

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
            tokens: tokens,
            instructions: vec![],
            value: 0,
            loop_stack: vec![],
            compiling_instruction: CompilingInstruction::None,
        }
    }
    fn compile_compiling_instruction(&mut self) {
        if self.compiling_instruction == CompilingInstruction::None {
            return;
        }
        self.instructions.push(match self.compiling_instruction {
            CompilingInstruction::Move => Instruction::Move(self.value),
            CompilingInstruction::Increment => {
                if self.value.is_positive() {
                    Instruction::Increment(self.value as u8)
                } else {
                    Instruction::Decrement(self.value.abs() as u8)
                }
            }
            CompilingInstruction::None => unreachable!(),
        });
        self.compiling_instruction = CompilingInstruction::None;
        self.value = 0
    }
    fn forward_backward(&mut self, token: Token) {
        if self.compiling_instruction != CompilingInstruction::Move {
            self.compile_compiling_instruction();
            self.compiling_instruction = CompilingInstruction::Move;
        }
        self.value += if let Token::Forward = token { 1 } else { -1 };
    }
    fn increment_decrement(&mut self, token: Token) {
        if self.compiling_instruction != CompilingInstruction::Increment {
            self.compile_compiling_instruction();
            self.compiling_instruction = CompilingInstruction::Increment;
        }
        self.value += if let Token::Increment = token { 1 } else { -1 };
    }
    fn start_loop(&mut self) {
        self.loop_stack.push(self.instructions.len());
        self.instructions.push(Instruction::LoopStart(0)) // temp 0
    }
    fn end_loop(&mut self) {
        let loop_start = self.loop_stack.pop().expect("loop end without start"); // Index it should jump to in order to restart the loop
        let loop_end = self.instructions.len(); // Index it should jump to in order to skip the loop

        let replacement = if loop_end - loop_start - 1 == 1 {
            // Only one type of instruction there
            match self.instructions[loop_start + 1] {
                Instruction::Decrement(decrement) => {
                    self.instructions.remove(loop_start + 1);

                    Instruction::DecrementLoop(decrement)
                }
                Instruction::Increment(increment) => {
                    self.instructions.remove(loop_start + 1);

                    Instruction::IncrementLoop(increment)
                }
                Instruction::Move(offset) => {
                    self.instructions.remove(loop_start + 1);

                    Instruction::MoveLoop(offset)
                }
                _ => {
                    self.instructions.push(Instruction::LoopEnd(loop_start));
                    Instruction::LoopStart(loop_end)
                }
            }
        } else {
            self.instructions.push(Instruction::LoopEnd(loop_start));
            Instruction::LoopStart(loop_end)
        };
        self.instructions[loop_start] = replacement;
    }

    pub fn compile(&mut self) -> &Vec<Instruction> {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Forward | Token::Backward => self.forward_backward(token),
                Token::Increment | Token::Decrement => self.increment_decrement(token),
                Token::Comment => {}

                Token::LoopStart => {
                    self.compile_compiling_instruction();
                    self.start_loop()
                }
                Token::LoopEnd => {
                    self.compile_compiling_instruction();
                    self.end_loop()
                }
                Token::Input => {
                    self.compile_compiling_instruction();
                    self.instructions.push(Instruction::Input)
                }
                Token::Output => {
                    self.compile_compiling_instruction();
                    self.instructions.push(Instruction::Output)
                }
            }
        }

        self.compile_compiling_instruction();

        if !self.loop_stack.is_empty() {
            panic!("Unclosed loop");
        }

        &self.instructions
    }
}
