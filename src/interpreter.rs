use crate::compile::Instruction;
use std::io::{stdin, stdout, Read, Write};
pub fn execute(instructions: &Vec<Instruction>) -> Vec<u8> {
    let mut stdout = stdout().lock();

    let mut memory: Vec<u8> = vec![0; 50];
    let mut pointer: isize = 0;

    let mut cell = unsafe { memory.get_unchecked_mut(pointer as usize) };

    let mut instruction_index = 0;
    loop {
        match unsafe {instructions.get_unchecked(instruction_index)} {
            Instruction::Move(offset) => {
                pointer += offset;
                if pointer as usize >= memory.len() {
                    memory.resize(pointer as usize + 10, 0)
                }
                cell = unsafe { memory.get_unchecked_mut(pointer as usize) };
            }
            Instruction::Increment(increment) => *cell += increment,
            Instruction::Decrement(decrement) => *cell -= decrement,

            Instruction::DecrementLoop(decrement) => {
                if *cell % *decrement == 0 {
                    *cell = 0
                } else {
                    panic!("Infinite loop detected")
                }
            }
            Instruction::IncrementLoop(increment) => {
                if *cell % *increment == 0 {
                    *cell = 0
                } else {
                    panic!("Infinite loop detected")
                }
            }
            Instruction::MoveLoop(offset) => {
                while *cell != 0 {
                    pointer += offset;
                    if pointer as usize >= memory.len() {
                        memory.resize(pointer as usize + 10, 0)
                    }
                    cell = unsafe { memory.get_unchecked_mut(pointer as usize) };
                }
            }
            Instruction::LoopStart(loop_end) => {
                if *cell == 0 {
                    instruction_index = *loop_end;
                    continue;
                }
            }
            Instruction::LoopEnd(loop_start) => {
                if *cell != 0 {
                    instruction_index = *loop_start;
                    continue;
                }
            }
            Instruction::Output => write!(stdout, "{}", *cell as char).unwrap(),
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                stdin().read_exact(&mut input).unwrap();
                *cell = input[0];
            }
            Instruction::Stop => break
        }
        instruction_index += 1;
    }

    memory
}
