use crate::compile::Instruction;
use std::io::{stdin, stdout, Read, Write};
pub fn execute(instructions: &[Instruction]) -> Vec<u8> {
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();

    let mut memory: Vec<u8> = vec![0; 32];
    let mut pointer: usize = 0;

    let mut instruction_index = 0;
    loop {
        match unsafe { instructions.get_unchecked(instruction_index) } {
            Instruction::LoopStart(loop_exit) => {
                if unsafe { *memory.get_unchecked_mut(pointer) } == 0 {
                    instruction_index = *loop_exit;
                    continue;
                }
            }
            Instruction::LoopEnd(loop_body) => {
                if unsafe { *memory.get_unchecked_mut(pointer) } != 0 {
                    instruction_index = *loop_body;
                    continue;
                }
            }

            Instruction::Forward(offset) => {
                pointer += offset;
                if pointer >= memory.len() {
                    memory.resize(pointer + 16, 0);
                }
            }
            Instruction::Backward(offset) => {
                pointer -= offset;
            }
            Instruction::Increment(increment) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = cell.wrapping_add(*increment);
            }
            Instruction::Decrement(decrement) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = cell.wrapping_sub(*decrement);
            }
            Instruction::SetZero => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = 0;
            }

            Instruction::ForwardLoop(offset) => {
                while unsafe { *memory.get_unchecked(pointer) } != 0 {
                    pointer += offset;
                    if pointer >= memory.len() {
                        memory.resize(pointer + 16, 0);
                    }
                }
            }
            Instruction::BackwardLoop(offset) => {
                while unsafe { *memory.get_unchecked(pointer) } != 0 {
                    pointer -= offset;
                }
            }
            Instruction::Output => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                write!(stdout, "{}", *cell as char).unwrap()
            }

            Instruction::IncrementLoop(increment) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                if *cell % *increment == 0 {
                    *cell = 0;
                } else {
                    panic!("Infinite loop detected")
                }
            }

            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                stdin.read_exact(&mut input).unwrap();
                let cell = unsafe { memory.get_unchecked_mut(pointer) };
                *cell = input[0];
            }
            Instruction::Stop => break,
        }
        instruction_index += 1;
    }

    memory
}
