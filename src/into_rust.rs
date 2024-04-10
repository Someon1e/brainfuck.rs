use crate::compile::Instruction;

pub fn to_rust(instructions: &[Instruction]) -> String {
    let mut code = String::with_capacity(5);

    let mut indent_level = 0;
    macro_rules! indent {
        () => {
            for _ in 0..indent_level {
                code.push('\t');
            }
        };
    }
    macro_rules! push_str {
        ($text:expr) => {
            code.push_str($text);
        };
    }
    macro_rules! indented_push {
        ($text:expr) => {
            indent!();
            push_str!($text);
        };
    }
    macro_rules! forward {
        ($offset:expr) => {
            push_str!("pointer += ");
            push_str!(&$offset.to_string());
            push_str!(";\n");

            indented_push!("if pointer >= memory.len() {\n");

            indent_level += 1;
            indented_push!("memory.resize(pointer + 16, Wrapping(0));\n");
            indent_level -= 1;

            indented_push!("}\n");
        };
    }

    push_str!("use std::io::{stdin, Read};\n");
    push_str!("use std::num::Wrapping;\n");

    push_str!("fn main() {\n");
    indent_level += 1;
    indented_push!("let mut stdin = stdin().lock();\n");
    indented_push!("let mut pointer: usize = 0;\n");
    indented_push!("let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 32];\n");

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
                indented_push!("pointer -= ");
                push_str!(&offset.to_string());
                push_str!(";\n");
            }
            Instruction::Increment(increment) => {
                indented_push!("memory[pointer] += ");
                push_str!(&increment.to_string());
                push_str!(";\n");
            }
            Instruction::Decrement(decrement) => {
                indented_push!("memory[pointer] -= ");
                push_str!(&decrement.to_string());
                push_str!(";\n");
            }
            Instruction::SetZero => {
                indented_push!("memory[pointer] = Wrapping(0);\n");
            }
            Instruction::LoopStart(_loop_end) => {
                indented_push!("while unsafe { memory.get_unchecked(pointer).0 } != 0 {\n");
                indent_level += 1;
            }
            Instruction::LoopEnd(_loop_start) => {
                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::IncrementLoop(value) => {
                indented_push!("let cell = unsafe { memory.get_unchecked_mut(pointer).0 };\n");

                indented_push!("if *cell % ");
                push_str!(&value.to_string());
                push_str!(" == 0 {");
                push_str!("*cell = 0 }");
                push_str!("else {panic!(\"Infinite loop detected\")}");
            }
            Instruction::ForwardLoop(offset) => {
                indented_push!("while unsafe { memory.get_unchecked(pointer).0 } != 0 {\n");
                indent_level += 1;

                indent!();
                forward!(offset);

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::BackwardLoop(offset) => {
                indented_push!("while unsafe { memory.get_unchecked(pointer).0 } != 0 {pointer -= ");
                push_str!(&offset.to_string());
                push_str!("}\n");
            }
            Instruction::Output => {
                indented_push!(
                    "print!(\"{}\", unsafe { memory.get_unchecked(pointer).0 } as char);\n"
                );
            }
            Instruction::Input => {
                indented_push!("let mut input: [u8; 1] = [0; 1];\n");

                indented_push!("stdin.read_exact(&mut input).unwrap();\n");

                indented_push!("memory[pointer] = Wrapping(input[0]);\n");
            }
            Instruction::Stop => break,
        }
        instruction_index += 1;
    }

    indent_level -= 1;
    indent!();
    code.push('}');

    code
}
