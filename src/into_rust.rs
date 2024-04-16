use crate::MEMORY_RESIZE_AMOUNT;
use crate::{compile::Instruction, INITIAL_MEMORY_CAPACITY};

#[allow(clippy::too_many_lines)]
pub fn to_rust(instructions: &[Instruction]) -> String {
    let mut code = String::with_capacity(256);

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
            push_str!("point_right!(");
            push_str!(&$offset.to_string());
            push_str!(");\n");
        };
    }

    push_str!("use std::io::{stdin, Read};\n");
    push_str!("use std::num::Wrapping;\n");

    push_str!("fn main() {\n");
    indent_level += 1;
    indented_push!("let mut stdin = stdin().lock();\n");
    indented_push!("let mut pointer: usize = 0;\n");

    indented_push!("let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); ");
    push_str!(&INITIAL_MEMORY_CAPACITY.to_string());
    push_str!("];\n");

    push_str!(
        "\tmacro_rules! cell {
\t\t() => {
\t\t\tunsafe { memory.get_unchecked(pointer) }
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! mut_cell {
\t\t($position:expr) => {
\t\t\tunsafe { memory.get_unchecked_mut($position) }
\t\t};
\t\t() => {
\t\t\tmut_cell!(pointer)
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! increment {
\t\t($number:expr) => {
\t\t\t*mut_cell!() += $number;
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! decrement {
\t\t($number:expr) => {
\t\t\t*mut_cell!() -= $number;
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! set_cell {
\t\t($number:expr) => {
\t\t\t*mut_cell!() = Wrapping($number);
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! point_right {
\t\t($offset:expr) => {
\t\t\tpointer = pointer.wrapping_add($offset);
\t\t\tif pointer >= memory.len() {
\t\t\t\tmemory.resize(pointer + 16, Wrapping(0));
\t\t\t}
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! point_left {
\t\t($offset:expr) => {
\t\t\tpointer = pointer.wrapping_sub($offset)
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! output {
\t\t() => {
\t\t\tprint!(\"{}\", cell!().0 as char);
\t\t};
\t}\n"
    );

    push_str!(
        "\tmacro_rules! cell_is_not_zero {
\t\t() => {
\t\t\tcell!().0 != 0
\t\t};
\t}\n"
    );

    let mut instruction_index = 0;
    loop {
        match unsafe { instructions.get_unchecked(instruction_index) } {
            Instruction::Forward(offset) => {
                indent!();
                forward!(offset);
            }
            Instruction::Backward(offset) => {
                indented_push!("point_left!(");
                push_str!(&offset.to_string());
                push_str!(");\n");
            }
            Instruction::Increment(increment) => {
                indented_push!("increment!(");
                push_str!(&increment.to_string());
                push_str!(");\n");
            }
            Instruction::Decrement(decrement) => {
                indented_push!("decrement!(");
                push_str!(&decrement.to_string());
                push_str!(");\n");
            }
            Instruction::SetZero => {
                indented_push!("set_cell!(0);\n");
            }
            Instruction::SetCell(value) => {
                indented_push!("set_cell!(");
                push_str!(&value.to_string());
                push_str!(");\n");
            }
            Instruction::LoopStart(_loop_end) => {
                indented_push!("while cell_is_not_zero!() {\n");
                indent_level += 1;
            }
            Instruction::LoopEnd(_loop_start) => {
                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::IncrementLoop(value) => {
                indented_push!("let cell = mut_cell!();\n");

                indented_push!("if cell.0 % ");
                push_str!(&value.to_string());
                push_str!(" == 0 {\n");

                indent_level += 1;
                indented_push!("*cell = Wrapping(0)\n");
                indent_level -= 1;

                indented_push!("} else {\n");

                indent_level += 1;
                indented_push!("panic!(\"Infinite loop detected\")\n");
                indent_level -= 1;

                indented_push!("}\n");
            }
            Instruction::MultiplyForward(offset, multiplier) => {
                indented_push!("let cell = *cell!();\n");

                indented_push!("if pointer + ");
                push_str!(&offset.to_string());
                push_str!(" >= memory.len() {\n");

                indent_level += 1;
                indented_push!("memory.resize(pointer + ");
                push_str!(&(*offset + MEMORY_RESIZE_AMOUNT).to_string());
                push_str!(", Wrapping(0));\n");
                indent_level -= 1;

                indented_push!("}\n");

                indented_push!("*mut_cell!(pointer + ");
                push_str!(&offset.to_string());
                push_str!(") += cell");
                if *multiplier != 1 {
                    push_str!(" * Wrapping(");
                    push_str!(&multiplier.to_string());
                    code.push(')');
                }
                push_str!(";\n");
            }
            Instruction::ForwardLoop(offset) => {
                indented_push!("while cell_is_not_zero!() {\n");
                indent_level += 1;

                indent!();
                forward!(offset);

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::BackwardLoop(offset) => {
                indented_push!("while cell_is_not_zero!() {\n");
                indent_level += 1;

                indented_push!("point_left!(");
                push_str!(&offset.to_string());
                push_str!(")\n");

                indent_level -= 1;
                indented_push!("}\n");
            }
            Instruction::Output => {
                indented_push!("output!();\n");
            }
            Instruction::Input => {
                indented_push!("let mut input: [u8; 1] = [0; 1];\n");

                indented_push!("stdin.read_exact(&mut input).unwrap();\n");

                indented_push!("set_cell!(input[0]);\n");
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
