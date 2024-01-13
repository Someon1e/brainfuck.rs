use crate::compile::Instruction;

pub fn to_rust(instructions: &Vec<Instruction>) -> String {
    let mut code = vec![
        String::from("let mut pointer: usize = 0;"),
        String::from("let mut memory: Vec<u8> = vec![0; 50];"),
    ];

    for instruction in instructions {
        code.push(match instruction {
            Instruction::Move(offset) => {
                if offset.is_positive() {
                format!(
"pointer += {offset};
if pointer >= memory.len() {{
memory.resize(pointer + 10, 0)
}}")
                } else {
                    // really hope it doesn't wrap around
                    // TODO: fix above
                    format!("pointer -= {};", offset.abs())
                }
            }
            Instruction::Increment(increment) => {
                format!("memory[pointer] += {increment};")
            }
            Instruction::Decrement(decrement) => {
                format!("memory[pointer] -= {decrement};")
            }
            Instruction::DecrementLoop(decrement) => {
                format!(
"let cell = unsafe {{ memory.get_unchecked_mut(pointer) }};
if *cell % {decrement} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}")}
            Instruction::IncrementLoop(increment) => {
                format!(
"let cell = unsafe {{ memory.get_unchecked_mut(pointer) }};
if *cell % {increment} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}")     
            }
            Instruction::MoveLoop(offset) => {
                if offset.is_positive() {
                    format!(
"while unsafe {{ *memory.get_unchecked(pointer) }} != 0 {{
pointer += {offset};
if pointer >= memory.len() {{
memory.resize(pointer + 10, 0)
}}
}}")
                    } else {
                    format!(
"while unsafe {{ *memory.get_unchecked(pointer) }} != 0 {{
pointer -= {};
}}", offset.abs())
                }
            }
            Instruction::LoopStart(_loop_end) => {
                r#"while unsafe { *memory.get_unchecked(pointer) } != 0 {"#.to_owned()
            }
            Instruction::LoopEnd(_loop_start) => {
                r#"}"#.to_owned()
            }
            Instruction::Output => {
                r#"print!("{}", unsafe { *memory.get_unchecked(pointer) } as char);"#.to_owned()
            }
            Instruction::Input => {
                unimplemented!()
            }
        })
    }

    format!("fn main() {{\n{}\n}}", code.join("\n"))
}
