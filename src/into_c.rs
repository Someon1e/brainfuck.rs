use core::iter;

use crate::MEMORY_RESIZE_AMOUNT;
use crate::{compile::Instruction, INITIAL_MEMORY_CAPACITY};

/// Compiles instructions into C
#[allow(clippy::too_many_lines)]
pub fn to_c(instructions: &[Instruction]) -> String {
    let mut code = String::with_capacity(2048);

    let mut indent_level = 0;
    macro_rules! indent {
        () => {
            code.extend(iter::repeat('\t').take(indent_level));
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
            push_str!("point_right(");
            push_str!(&$offset.to_string());
            push_str!(");\n");
        };
    }

    push_str!("#include <stdio.h>\n");
    push_str!("#include <stdint.h>\n");
    push_str!("#include <stdlib.h>\n");
    push_str!("#include <string.h>\n");
    push_str!("uint8_t* memory = NULL;\n");
    push_str!("int pointer = 0;\n");
    push_str!("int cell = 0;\n");
    push_str!("int memory_size = ");
    push_str!(&INITIAL_MEMORY_CAPACITY.to_string());
    push_str!(";\n");
    push_str!(
        "void grow_memory(size_t new_size) {
\tmemory = (uint8_t*)realloc(memory, new_size * sizeof(uint8_t));
\tif (memory == NULL) {
\t\texit(1);
\t}
\tmemset(memory + memory_size, 0, (new_size - memory_size) * sizeof(uint8_t));
\tmemory_size = new_size;
}\n"
    );

    push_str!(
        "inline void point_right(size_t offset) {
\tpointer += offset;
\tif (pointer >= memory_size) {
\t\tgrow_memory(pointer + "
    );
    push_str!(&MEMORY_RESIZE_AMOUNT.to_string());

    push_str!(");\n\t}\n}\n");

    push_str!("int main() {\n");

    push_str!(
        "\tmemory = (uint8_t*)calloc(memory_size, sizeof(uint8_t));
\tif (memory == NULL) {
\t\texit(1);
\t}\n"
    );
    indent_level += 1;
    let mut instruction_index = 0;
    loop {
        match unsafe { instructions.get_unchecked(instruction_index) } {
            Instruction::Forward(offset) => {
                indent!();
                forward!(offset);
            }
            Instruction::Backward(offset) => {
                indented_push!("pointer -= ");
                push_str!(&offset.to_string());
                push_str!(";\n");
            }
            Instruction::Increment(increment) => {
                indented_push!("memory[pointer] += ");
                push_str!(&increment.to_string());
                push_str!(";\n");
            }
            Instruction::SetCell(value) => {
                indented_push!("memory[pointer] = ");
                push_str!(&value.to_string());
                push_str!(";\n");
            }
            Instruction::LoopStart(_loop_end) => {
                indented_push!("while (memory[pointer] != 0) {\n");
                indent_level += 1;
            }
            Instruction::LoopEnd(_loop_start) => {
                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::IncrementLoop(value) => {
                indented_push!("cell = memory[pointer];\n");

                indented_push!("while (cell != 0) {\n");

                indent_level += 1;
                indented_push!("cell += ");
                push_str!(&value.to_string());
                push_str!(";\n");
                indent_level -= 1;

                indented_push!("}\n");

                indented_push!("memory[pointer] = cell;\n");
            }
            Instruction::MultiplyForward(offset, multiplier) => {
                indented_push!("cell = memory[pointer];\n");

                indented_push!("if (cell != 0) {\n");
                indent_level += 1;

                indented_push!("if (pointer + ");
                push_str!(&offset.to_string());
                push_str!(" >= memory_size) {\n");

                indent_level += 1;
                indented_push!("grow_memory(pointer + ");
                push_str!(&(*offset as usize + MEMORY_RESIZE_AMOUNT).to_string());
                push_str!(");\n");
                indent_level -= 1;

                indented_push!("}\n");

                indented_push!("memory[pointer + ");
                push_str!(&offset.to_string());
                push_str!("] += cell");
                if *multiplier != 1 {
                    push_str!(" * ");
                    push_str!(&multiplier.to_string());
                }
                push_str!(";\n");

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::MultiplyBackward(offset, multiplier) => {
                indented_push!("cell = memory[pointer];\n");

                indented_push!("if (cell != 0) {\n");
                indent_level += 1;

                indented_push!("memory[pointer - ");
                push_str!(&offset.to_string());
                push_str!("] += cell");
                if *multiplier != 1 {
                    push_str!(" * ");
                    push_str!(&multiplier.to_string());
                }
                push_str!(";\n");

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::ForwardLoop(offset) => {
                indented_push!("while (memory[pointer] != 0) {\n");
                indent_level += 1;

                indent!();
                forward!(offset);

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::BackwardLoop(offset) => {
                indented_push!("while (memory[pointer] != 0) {\n");
                indent_level += 1;

                indented_push!("pointer -= ");
                push_str!(&offset.to_string());
                push_str!(";\n");

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::Output => {
                indented_push!("putchar(memory[pointer]);\n");
            }
            Instruction::Input => {
                indented_push!("memory[pointer] = getchar();\n");
            }
            Instruction::Stop => break,
        }
        instruction_index += 1;
    }
    indented_push!("free(memory);\n");
    indented_push!("return 0;\n");

    indent_level -= 1;
    indent!();
    code.push('}');

    code
}
