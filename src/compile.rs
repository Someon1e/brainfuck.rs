use crate::lexer::Token;

#[derive(Debug)]
pub enum Instruction {
    Forward(usize),
    Backward(usize),

    Increment(u8),

    SetZero,
    SetCell(u8),
    IncrementLoop(u8),

    MultiplyForward(usize, u8),
    MultiplyBackward(usize, u8),

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
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
    instructions: Vec<Instruction>,
    loop_stack: Vec<usize>,
    compiling_instruction: CompilingInstruction,
    value: isize,
    cell_guarantee: Option<u8>,
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: impl Iterator<Item = Token> + 'a) -> Self {
        Self {
            tokens: Box::new(tokens),
            instructions: vec![],
            loop_stack: vec![],
            compiling_instruction: CompilingInstruction::None,
            value: 0,
            cell_guarantee: Some(0),
        }
    }
    fn compile_compiling_instruction(&mut self) {
        match self.compiling_instruction {
            CompilingInstruction::None => return,

            CompilingInstruction::Move => {
                if self.value != 0 {
                    if self.value.is_positive() {
                        self.instructions
                            .push(Instruction::Forward(self.value as usize));
                    } else {
                        self.instructions
                            .push(Instruction::Backward(self.value.unsigned_abs()));
                    }
                    self.cell_guarantee = None;
                }
            }
            CompilingInstruction::Increment => {
                if self.value != 0 {
                    if let Some(cell_guarantee) = self.cell_guarantee {
                        self.cell_guarantee = Some(((cell_guarantee as isize) + self.value) as u8);
                        self.instructions
                            .push(Instruction::SetCell(self.cell_guarantee.unwrap()));
                    } else {
                        self.instructions.push(if self.value.is_positive() {
                            Instruction::Increment(self.value as u8)
                        } else {
                            Instruction::Increment(
                                0u8.wrapping_sub(self.value.unsigned_abs() as u8),
                            )
                        });
                    }
                }
            }
        };
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
        if self.cell_guarantee == Some(0) {
            let mut count = 1;
            for token in self.tokens.by_ref() {
                match token {
                    Token::LoopStart => {
                        count += 1;
                    }
                    Token::LoopEnd => {
                        count -= 1;
                    }
                    _ => {}
                }
                if count == 0 {
                    return;
                }
            }
            panic!("Unclosed loop")
        } else {
            self.loop_stack.push(self.instructions.len());
            self.instructions.push(Instruction::LoopStart(0)); // temp 0
            self.cell_guarantee = None;
        }
    }
    fn end_loop(&mut self) {
        let loop_start = self.loop_stack.pop().expect("loop end without start"); // Index of loop start instruction

        let loop_end = self.instructions.len(); // Index of loop end instruction

        if loop_end - loop_start - 1 == 0 {
            self.instructions.push(Instruction::LoopEnd(loop_start + 1));
            return;
        }
        if loop_end - loop_start - 1 == 1 {
            // Only one type of instruction there
            self.instructions[loop_start] = match *self.instructions.get(loop_start + 1).unwrap() {
                Instruction::Increment(value) => {
                    self.instructions.remove(loop_start + 1);

                    if value == 1 || value == u8::MAX {
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
            let multipliers = 'out: {
                let mut all_increments = rustc_hash::FxHashMap::default();
                let mut total_offset: isize = 0;
                let mut total_increment: u8 = 0;

                for index in loop_start + 1..loop_end {
                    // Iterate all instructions inside the loop
                    match self.instructions.get(index).unwrap() {
                        inner @ (Instruction::Forward(offset) | Instruction::Backward(offset)) => {
                            if total_increment != 0 {
                                all_increments.insert(
                                    total_offset,
                                    all_increments.get(&total_offset).unwrap_or(&0)
                                        + total_increment,
                                );
                                total_increment = 0;
                            }
                            if matches!(inner, Instruction::Forward(_)) {
                                total_offset += *offset as isize;
                            } else {
                                total_offset -= *offset as isize;
                            }
                        }
                        Instruction::Increment(increment) => {
                            total_increment += *increment;
                        }
                        _ => break 'out None,
                    }
                }
                if total_increment != 0 {
                    all_increments.insert(
                        total_offset,
                        all_increments.get(&total_offset).unwrap_or(&0) + total_increment,
                    );
                    #[allow(unused_assignments)]
                    {
                        total_increment = 0;
                    }
                }

                if !all_increments.is_empty() && // There are multipliers
                    total_offset == 0 && // We are on the starting cell
                    all_increments.remove(&0) == Some(u8::MAX)
                // It decrements the starting cell
                {
                    break 'out Some(all_increments);
                }

                None
            };

            if let Some(multipliers) = multipliers {
                self.instructions.truncate(loop_start);
                for (offset, multiplier) in multipliers {
                    if offset.is_positive() {
                        self.instructions.push(Instruction::MultiplyForward(
                            offset as usize,
                            multiplier as u8,
                        ));
                    } else {
                        self.instructions.push(Instruction::MultiplyBackward(
                            offset.abs() as usize,
                            multiplier as u8,
                        ));
                    }
                }
                self.instructions.push(Instruction::SetZero);
            } else {
                self.instructions.push(Instruction::LoopEnd(loop_start + 1));
                self.instructions[loop_start] = Instruction::LoopStart(loop_end + 1);
            }
        };

        self.cell_guarantee = Some(0);
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
                    self.cell_guarantee = None;
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
