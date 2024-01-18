use crate::compile::Instruction;

pub fn to_rust(instructions: &[Instruction]) -> String {
    let mut code = String::new();

    let mut indent_level = 0;
    macro_rules! indent {
        () => {
            for _ in 0..indent_level {
                code.push('\t');
            }
        }
    }
    macro_rules! push_str {
        ($text:expr) => {
            code.push_str($text)
        }
    }
    macro_rules! forward {
        ($offset:expr) => {
            push_str!("pointer += "); push_str!(&$offset.to_string()); push_str!(";\n");

            indent!();
            push_str!("if pointer >= memory.len() {\n");

            indent_level += 1;
                indent!();
                push_str!("memory.resize(pointer + 10, 0)\n");
            indent_level -= 1;

            indent!();
            push_str!("}\n");
        }
    }

    push_str!("use std::io::{stdin, Read};\n");

    push_str!("fn main() {\n");
    indent_level += 1;
    indent!(); push_str!("let mut stdin = stdin().lock();\n");
    indent!(); push_str!("let mut pointer: usize = 0;\n");
    indent!(); push_str!("let mut memory: Vec<u8> = vec![0; 50];\n");

    let mut instruction_index = 0;
    loop {
        match unsafe { instructions.get_unchecked(instruction_index) } {
            Instruction::Forward(offset) => {
                indent!();
                forward!(offset);
            }
            Instruction::Backward(offset) => {
                // really hope it doesn't wrap around
                // TODO: fix above
                indent!();
                push_str!("pointer -= ");
                push_str!(&offset.to_string());
                push_str!(";\n")
            }
            Instruction::Increment(increment) => {
                indent!();
                push_str!("memory[pointer] += ");
                push_str!(&increment.to_string());
                push_str!(";\n")
            }
            Instruction::Decrement(decrement) => {
                indent!();
                push_str!("memory[pointer] -= ");
                push_str!(&decrement.to_string());
                push_str!(";\n")
            }
            Instruction::SetZero => {
                indent!();
                push_str!("memory[pointer] = 0;\n")
            }
            Instruction::LoopStart(_loop_end) => {
                indent!();
                push_str!("while unsafe { *memory.get_unchecked(pointer) } != 0 {\n");
                indent_level += 1;
            }
            Instruction::LoopEnd(_loop_start) => {
                indent_level -= 1;
                indent!();
                push_str!("}\n")
            }
            Instruction::IncrementLoop(value) => {
                indent!();
                push_str!("let cell = unsafe { memory.get_unchecked_mut(pointer) };\n");

                indent!();
                push_str!("if *cell % "); push_str!(&value.to_string()); push_str!(" == 0 {"); push_str!("*cell = 0 }");
                push_str!("else {panic!(\"Infinite loop detected\")}");
            }
            Instruction::ForwardLoop(offset) => {
                indent!();
                push_str!("while unsafe { *memory.get_unchecked(pointer) } != 0 {\n");
                indent_level += 1;

                indent!();
                forward!(offset);

                indent_level -= 1;
                indent!();
                push_str!("}\n");
            }
            Instruction::BackwardLoop(offset) => {
                indent!();
                push_str!("while unsafe { *memory.get_unchecked(pointer) } != 0 {pointer -= ");
                push_str!(&offset.to_string());
                push_str!("}\n");
            }
            Instruction::Output => {
                indent!();
                push_str!("print!(\"{}\", unsafe { *memory.get_unchecked(pointer) } as char);\n")
            }
            Instruction::Input => {
                indent!();
                push_str!("let mut input: [u8; 1] = [0; 1];\n");

                indent!();
                push_str!("stdin.read_exact(&mut input).unwrap();\n");

                indent!();
                push_str!("memory[pointer] = input[0];\n")
            }
            Instruction::Stop => break,
        }
        instruction_index += 1
    }

    indent_level -= 1;
    indent!();
    code.push('}');

    code
}
