# Jasmine 2: Hacking `rustc` for Fun and Profit

Version 1.0 was a weird language, made with constraints because "type evaluator too hard." It had its own parser (for Rust), its own AST (for Rust), and its own
compiler checks (for Rust).

Instead, why not piggyback off of the hard work already done by the wonderful developers of Rust. Unknown to me at the time, the language exposes an IR with all of the type information we need. Instead of compiling rust from source, we can compile MIR (which already has all of the necessary type information evalutated). As a bonus, we'd actually be compiling actual Rust rather than some dummy language made to look like it. We'd also get access to the entire Rust ecosystem (at least, the no-std, no-alloc bits).

## Constraints

Obviously, not every Rust program can be implemented in Java. That's because Rust has things like C FFI, its own allocator, and manual memory management. The Rust standard library is based on having OS-level access, which Java simply does not give us. So, we can make the following constraints:

- All programs must be `#![no_std]`. Instead, we're going to make our own `jasmine-std` standard library, which will mimic Rust's as much as it can, but use Java primitives instead.
- All programs must be without alloc. Java doesn't give us OS-level access to an allocator, therefore we can't expose anything like that to the end user.
- All programs cannot have C-style FFI. Instead, we can make `extern "Java" {}` blocks which gives the programmer access to any Java primitives we do not already give them.
