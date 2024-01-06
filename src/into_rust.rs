use crate::compile::Instruction;

pub fn to_rust(instructions: Vec<Instruction>) -> String {
    let mut code = vec![
        String::from("let mut pointer: isize = 0;"),
        String::from("let mut memory: rustc_hash::FxHashMap<isize, u8> = rustc_hash::FxHashMap::default();"),
    ];

    for instruction in instructions {
        code.push(match instruction {
            Instruction::Move(offset) => {
                format!("pointer += {offset};").to_owned()
            }
            Instruction::Increment(increment) => {
                format!("memory.entry(pointer).and_modify(|value| *value += {increment}).or_insert({increment});").to_owned()
            }
            Instruction::Decrement(decrement) => {
                format!("memory.entry(pointer).and_modify(|value| *value -= {decrement}).or_insert({});", 0u8.wrapping_sub(decrement)).to_owned()
            }
            Instruction::DecrementLoop(decrement) => {
                format!(
"memory.entry(pointer).and_modify(|cell| {{
if *cell % {decrement} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}
}});").to_owned()}
            Instruction::IncrementLoop(increment) => {
                format!(
"memory.entry(pointer).and_modify(|cell| {{
if *cell % {increment} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}
}});").to_owned()     
            }
            Instruction::MoveLoop(offset) => {
                if offset.is_positive() {
                    format!("while *memory.get(&pointer).unwrap_or(&0) != 0 {{\npointer += {offset};\n}}").to_owned()
                } else {
                    format!("while *memory.get(&pointer).unwrap_or(&0) != 0 {{\npointer -= {};\n}}", offset.abs()).to_owned()
                }
            }
            Instruction::LoopStart(loop_end) => {
                format!("while *memory.get(&pointer).unwrap_or(&0) != 0 {{").to_owned()
            }
            Instruction::LoopEnd(loop_start) => {
                format!("}}").to_owned()
            }
            Instruction::Output => {
                format!("print!(\"{{}}\", *memory.get(&pointer).unwrap_or(&0) as char);").to_owned()
            }
            Instruction::Input => {
                format!("").to_owned()
            }
        })
    }

    format!("fn main() {{\n{}\n}}", code.join("\n"))
}
