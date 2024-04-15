# Overview
brainfuck.rs is a fast [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) executor in Rust.
It includes an interpreter and a brainfuck-to-rust transpiler.

# Install
```
cargo install --git https://github.com/Someon1e/brainfuck.rs brainfuck
```

# Usage
![image](https://github.com/Someon1e/brainfuck.rs/assets/142684596/d2579428-7875-4389-89fc-41d6aac9b95f)

# How?
It works by first processing the brainfuck program into tokens.
Then, the tokens are optimised into instructions (IR).

### Examples:
|Input                         |Tokens                                                                                                                                                                           |IR                                                   |
|------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------|
|`++-`                         |Increment, Increment, Decrement                                                                                                                                                  |Increment(1)                                         |
|`[-]`                         |LoopStart, Decrement, LoopEnd                                                                                                                                                    |SetZero                                              |
|`>a>b<`                       |Forward, Comment, Forward, Comment, Backward                                                                                                                                     |Forward(1)                                           |
|`[->+>+<<]`                   |LoopStart, Decrement, Forward, Increment, Forward, Increment, Backward, Backward, LoopEnd                                                                                        |MultiplyForward(1, 1), MultiplyForward(2, 1), SetZero|
|`[->+++>+++++++<<]`           |LoopStart, Decrement, Forward, Increment, Increment, Increment, Forward, Increment, Increment, Increment, Increment, Increment, Increment, Increment, Backward, Backward, LoopEnd|MultiplyForward(1, 3), MultiplyForward(2, 7), SetZero|
|`[.,]`                        |LoopStart, Input, Output, LoopEnd                                                                                                                                                |LoopStart(4), Input, Output, LoopEnd(1)              |
|`[>]`                         |LoopStart, Forward, LoopEnd                                                                                                                                                      |ForwardLoop(1)                                       |
|`[<<]`                        |LoopStart, Backward, Backward, LoopEnd                                                                                                                                           |BackwardLoop(2)                                      |

Finally, the IR is interpreted or transpiled.

# Files
- [`samples`](https://github.com/Someon1e/brainfuck.rs/tree/master/samples) contains example brainfuck programs.
- [`src/main`](https://github.com/Someon1e/brainfuck.rs/tree/master/src) contains the lexer, IR, interpreter, and transpiler.
