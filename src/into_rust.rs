use crate::compile::Instruction;

pub fn to_rust(instructions: &[Instruction]) -> String {
    let mut code = vec![
        String::from("let mut pointer: usize = 0;"),
        String::from("let mut memory: Vec<u8> = vec![0; 50];"),
    ];

    let mut instruction_index = 0;
    loop {
        code.push(
            match unsafe { instructions.get_unchecked(instruction_index) } {
                Instruction::Forward(offset) => {
                    format!(
                        "pointer += {offset};
if pointer >= memory.len() {{
memory.resize(pointer + 10, 0)
}}"
                    )
                }
                Instruction::Backward(offset) => {
                    // really hope it doesn't wrap around
                    // TODO: fix above
                    format!("pointer -= {};", offset)
                }
                Instruction::Increment(increment) => {
                    format!("memory[pointer] += {increment};")
                }
                Instruction::Decrement(decrement) => {
                    format!("memory[pointer] -= {decrement};")
                }
                Instruction::SetZero => "memory[pointer] = 0;".to_owned(),
                Instruction::IncrementLoop(value) => {
                    format!(
                        "let cell = unsafe {{ memory.get_unchecked_mut(pointer) }};
if *cell % {value} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}"
                    )
                }
                Instruction::ForwardLoop(offset) => {
                    format!(
                        "while unsafe {{ *memory.get_unchecked(pointer) }} != 0 {{
pointer += {offset};
if pointer >= memory.len() {{
memory.resize(pointer + 10, 0)
}}
}}"
                    )
                }
                Instruction::BackwardLoop(offset) => {
                    format!(
                        "while unsafe {{ *memory.get_unchecked(pointer) }} != 0 {{
pointer -= {offset};
}}"
                    )
                }
                Instruction::LoopStart(_loop_end) => {
                    r#"while unsafe { *memory.get_unchecked(pointer) } != 0 {"#.to_owned()
                }
                Instruction::LoopEnd(_loop_start) => r#"}"#.to_owned(),
                Instruction::Output => {
                    r#"print!("{}", unsafe { *memory.get_unchecked(pointer) } as char);"#.to_owned()
                }
                Instruction::Input => {
                    unimplemented!()
                }
                Instruction::Stop => break,
            },
        );
        instruction_index += 1
    }

    format!("fn main() {{\n{}\n}}", code.join("\n"))
}
