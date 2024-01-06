use crate::compile::Instruction;

pub fn to_rust(instructions: Vec<Instruction>) -> String {
    let mut code = vec![
        String::from("let mut cell: u8 = 0;"),
        String::from("let mut pointer: isize = 0;"),
        String::from("let mut memory = rustc_hash::FxHashMap::default();"),

        String::from("fn r#move(memory: &mut rustc_hash::FxHashMap<isize, u8>, cell: &mut u8, pointer: &mut isize, offset: isize) {"),
        String::from("memory.entry(*pointer).and_modify(|value| *value = *cell).or_insert(*cell);"),
        String::from("*pointer += offset;"),
        String::from("*cell = *memory.get(&pointer).unwrap_or(&0);"),
        String::from("}"),

        String::from("fn output(cell: u8) {"),
        String::from("print!(\"{}\", cell as char);"),
        String::from("}")
    ];

    for instruction in instructions {
        code.push(match instruction {
            Instruction::Move(offset) => {
                format!("r#move(&mut memory, &mut cell, &mut pointer, {offset});").to_owned()
            }
            Instruction::Increment(increment) => {
                format!("cell += {increment};").to_owned()
            }
            Instruction::Decrement(decrement) => {
                format!("cell -= {decrement};").to_owned()
            }
            Instruction::DecrementLoop(decrement) => {
                format!("if cell % {decrement} == 0 {{cell = 0}} else {{panic!(\"Infinite loop detected\")}}").to_owned()
            }
            Instruction::IncrementLoop(increment) => {
                format!("if cell % {increment} == 0 {{cell = 0}} else {{panic!(\"Infinite loop detected\")}}").to_owned()
            }
            Instruction::MoveLoop(offset) => {
                if offset.is_positive() {
                    format!("while *memory.get(&pointer).unwrap_or(&0) != 0 {{\npointer += {offset};\n}}").to_owned()
                } else {
                    format!("while *memory.get(&pointer).unwrap_or(&0) != 0 {{\npointer -= {};\n}}", offset.abs()).to_owned()
                }
            }
            Instruction::LoopStart(loop_end) => {
                format!("while cell != 0 {{").to_owned()
            }
            Instruction::LoopEnd(loop_start) => {
                format!("}}").to_owned()
            }
            Instruction::Output => {
                format!("output(cell);").to_owned()
            }
            Instruction::Input => {
                format!("").to_owned()
            }
        })
    }

    format!("fn main() {{\n{}\n}}", code.join("\n"))
}
