use crate::compile::Instruction;
use std::io::{stdin, Read};
pub fn execute(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut memory: Vec<u8> = vec![0; 50];
    let mut pointer: isize = 0;

    let mut instruction_index = 0;
    while let Some(instruction) = instructions.get(instruction_index) {
        match instruction {
            Instruction::Move(offset) => {
                pointer += offset;
                if pointer as usize >= memory.len() {
                    memory.resize(pointer as usize + 10, 0)
                }
            }
            Instruction::Increment(increment) => memory[pointer as usize] += increment,
            Instruction::Decrement(decrement) => memory[pointer as usize] -= decrement,

            Instruction::DecrementLoop(decrement) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer as usize) };
                if *cell % *decrement == 0 {
                    *cell = 0
                } else {
                    panic!("Infinite loop detected")
                }
            }
            Instruction::IncrementLoop(increment) => {
                let cell = unsafe { memory.get_unchecked_mut(pointer as usize) };
                if *cell % *increment == 0 {
                    *cell = 0
                } else {
                    panic!("Infinite loop detected")
                }
            }
            Instruction::MoveLoop(offset) => {
                while unsafe { *memory.get_unchecked(pointer as usize) } != 0 {
                    pointer += offset;
                    if pointer as usize >= memory.len() {
                        memory.resize(pointer as usize + 10, 0)
                    }
                }
            }
            Instruction::LoopStart(loop_end) => {
                let cell = unsafe { *memory.get_unchecked(pointer as usize) };
                if cell == 0 {
                    instruction_index = *loop_end;
                    continue;
                }
            }
            Instruction::LoopEnd(loop_start) => {
                let cell = unsafe { *memory.get_unchecked(pointer as usize) };
                if cell != 0 {
                    instruction_index = *loop_start;
                    continue;
                }
            }
            Instruction::Output => print!("{}", unsafe { *memory.get_unchecked(pointer as usize) } as char),
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                stdin().read_exact(&mut input).unwrap();
                *memory.get_mut(pointer as usize).unwrap() = input[0];
            }
        }
        instruction_index += 1;
    }

    memory
}
