use crate::lexer::Token;

#[derive(Debug)]
pub enum Instruction {
    Forward(usize),
    Backward(usize),

    Increment(u8),
    Decrement(u8),

    SetZero,
    SetCell(u8),
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
                            Instruction::Decrement(self.value.unsigned_abs() as u8)
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
                Instruction::Decrement(1)
            ) {
                let mut compiling_multiplier = 0;
                let mut compiling_multipliers = Vec::with_capacity(2);
                let mut total_offset = 0;
                for index in loop_start + 2..loop_end {
                    // Iterate all instructions inside the loop, except the first
                    let inner = self.instructions.get(index).unwrap();
                    match inner {
                        Instruction::Forward(offset) | Instruction::Backward(offset) => {
                            if compiling_multiplier != 0 {
                                compiling_multipliers.push((total_offset, compiling_multiplier));
                                compiling_multiplier = 0;
                            }

                            if matches!(inner, Instruction::Forward(_)) {
                                total_offset += offset;
                            } else {
                                total_offset -= offset;

                                if index == loop_end - 1 // This should be the last instruction
                                && total_offset == 0 // and we should be at the starting cell
                                && !compiling_multipliers.is_empty()
                                // and there should be multipliers
                                {
                                    multipliers = Some(compiling_multipliers);
                                }
                                break;
                            }
                        }
                        Instruction::Increment(increment) => compiling_multiplier += increment,
                        _ => break,
                    }
                }
            }

            if let Some(multipliers) = multipliers {
                self.instructions.truncate(loop_start);
                for (offset, multiplier) in multipliers {
                    self.instructions
                        .push(Instruction::MultiplyForward(offset, multiplier));
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
