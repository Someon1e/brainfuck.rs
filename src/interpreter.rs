use rustc_hash::FxHashMap;

use crate::compile::Instruction;
use std::io::{stdin, Read};
pub fn execute(instructions: Vec<Instruction>) -> FxHashMap<isize, u8> {
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
            Instruction::MoveLoop(offset) => {
                while *memory.get(&pointer).unwrap_or(&0) != 0 {
                    pointer += offset;
                }
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
