use crate::{compile::Instruction, INITIAL_MEMORY_CAPACITY, MEMORY_RESIZE_AMOUNT};
use core::num::Wrapping;
use std::io::{stdin, stdout, Read, Write};

/// Interprets instructions.
pub fn execute(instructions: &[Instruction]) -> Vec<Wrapping<u8>> {
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();

    let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); INITIAL_MEMORY_CAPACITY];
    let mut pointer: usize = 0;

    let mut instruction_index = 0;
    loop {
        match unsafe { instructions.get_unchecked(instruction_index) } {
            Instruction::Forward(offset) => {
                pointer = pointer.wrapping_add(*offset as usize);
                if pointer >= memory.len() {
                    memory.resize(pointer + MEMORY_RESIZE_AMOUNT, Wrapping(0));
                }
            }
            Instruction::Backward(offset) => {
                pointer = pointer.wrapping_sub(*offset as usize);
            }

            Instruction::Increment(increment) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell += increment;
            }

            Instruction::LoopStart(loop_exit) => {
                if unsafe { memory.get_unchecked_mut(pointer).0 } == 0 {
                    instruction_index = *loop_exit as usize;
                    continue;
                }
            }
            Instruction::LoopEnd(loop_body) => {
                if unsafe { memory.get_unchecked_mut(pointer).0 } != 0 {
                    instruction_index = *loop_body as usize;
                    continue;
                }
            }

            Instruction::SetCell(value) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = Wrapping(*value);
            }

            Instruction::MultiplyForward(offset, multiplier) => {
                let cell = unsafe { *memory.get_unchecked(pointer) };
                if cell != Wrapping(0) {
                    if pointer + *offset as usize >= memory.len() {
                        memory.resize(
                            pointer + *offset as usize + MEMORY_RESIZE_AMOUNT,
                            Wrapping(0),
                        );
                    }

                    *unsafe { memory.get_unchecked_mut(pointer + *offset as usize) } +=
                        cell * Wrapping(*multiplier);
                }
            }

            Instruction::MultiplyBackward(offset, multiplier) => {
                let cell = unsafe { *memory.get_unchecked(pointer) };

                if cell != Wrapping(0) {
                    *unsafe { memory.get_unchecked_mut(pointer - *offset as usize) } +=
                        cell * Wrapping(*multiplier);
                }
            }

            Instruction::ForwardLoop(offset) => {
                while unsafe { memory.get_unchecked(pointer).0 } != 0 {
                    pointer += *offset as usize;
                    if pointer >= memory.len() {
                        memory.resize(pointer + MEMORY_RESIZE_AMOUNT, Wrapping(0));
                        break;
                    }
                }
            }
            Instruction::BackwardLoop(offset) => {
                while unsafe { memory.get_unchecked(pointer).0 } != 0 {
                    pointer -= *offset as usize;
                }
            }
            Instruction::Output => {
                let cell = unsafe { memory.get_unchecked(pointer) };
                write!(stdout, "{}", cell.0 as char).unwrap();
            }

            Instruction::IncrementLoop(increment) => {
                let mut cell = unsafe { *memory.get_unchecked(pointer) };
                while cell != Wrapping(0) {
                    cell += *increment;
                }
                *unsafe { memory.get_unchecked_mut(pointer) } = cell;
            }

            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                stdin.read_exact(&mut input).unwrap();
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = Wrapping(input[0]);
            }
            Instruction::Stop => break,
        }
        instruction_index += 1;
    }

    memory
}
