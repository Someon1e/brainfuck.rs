use crate::compile::Instruction;

pub fn to_rust(instructions: Vec<Instruction>) -> String {
    let mut code = vec![
        String::from("let mut pointer: isize = 0;"),
        String::from("let mut memory: Vec<u8> = vec![0; 100];"),
    ];

    for instruction in instructions {
        code.push(match instruction {
            Instruction::Move(offset) => {
                if offset.is_positive() {
                format!(
"pointer += {offset};
if pointer as usize >= memory.len() {{
memory.resize(memory.len() + {offset} as usize, 0)
}}").to_owned()
                } else {
                format!(
"pointer -= {0};
if pointer as usize >= memory.len() {{
memory.resize(memory.len() + {0} as usize, 0)
}}", offset.abs()).to_owned()
                }
            }
            Instruction::Increment(increment) => {
                format!("memory[pointer as usize] += {increment};").to_owned()
            }
            Instruction::Decrement(decrement) => {
                format!("memory[pointer as usize] -= {decrement};").to_owned()
            }
            Instruction::DecrementLoop(decrement) => {
                format!(
"let cell = unsafe {{ memory.get_unchecked_mut(pointer as usize) }};
if *cell % {decrement} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}").to_owned()}
            Instruction::IncrementLoop(increment) => {
                format!(
"let cell = unsafe {{ memory.get_unchecked_mut(pointer as usize) }};
if *cell % {increment} == 0 {{
*cell = 0
}} else {{
panic!(\"Infinite loop detected\")
}}").to_owned()     
            }
            Instruction::MoveLoop(offset) => {
                if offset.is_positive() {
                    format!(
"while unsafe {{ *memory.get_unchecked(pointer as usize) }} != 0 {{
pointer += {offset};
if pointer as usize >= memory.len() {{
memory.resize(memory.len() + {offset} as usize, 0)
}}
}}").to_owned()
                    } else {
                    format!(
"while unsafe {{ *memory.get_unchecked(pointer as usize) }} != 0 {{
pointer -= {};
}}", offset.abs()).to_owned()
                }
            }
            Instruction::LoopStart(_loop_end) => {
                r#"while unsafe { *memory.get_unchecked(pointer as usize) } != 0 {"#.to_owned()
            }
            Instruction::LoopEnd(_loop_start) => {
                r#"}"#.to_owned()
            }
            Instruction::Output => {
                r#"print!("{}", unsafe { *memory.get_unchecked(pointer as usize) } as char);"#.to_owned()
            }
            Instruction::Input => {
                unimplemented!()
            }
        })
    }

    format!("fn main() {{\n{}\n}}", code.join("\n"))
}
