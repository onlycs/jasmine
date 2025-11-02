# The Jasmine Programming Language

I don't like java. The problem arises when my AP CS course forces me to use it. In retaliation, I created this -- a Rust to Java compiler.

## Features

- [x] Functions
- [x] Loops
- [x] Variables
- [x] Expressions
- [x] Statements
- [x] Structures
- [x] Functions on structures (impls)
- [ ] Traits (interfaces)
- [x] Enums
- [x] Auto-Expansion of `fn main()` to `public static void main(String[] args)`
- [x] Math
- [x] If-let statements (for enums only)
- [x] Match statements (for enums only)
- [x] Builtins
- [ ] Type casting (not planned)
- [x] Arrays (kinda)

## So what's changed from Rust
- Macros are just functions
- No modules
- No use statements (`java.util.*` is imported by default)
- Arrays are different
- Character literals only sometimes work
- Can only use literal for rhs of range
- No traits, type casting
- No type inferencing (must declare types)
- Closures use custom types in Java
- Removed rust std and core
- A whole lot more

## Usage
```
A Rust-like programming langauge that transpiles to Java

Usage: jasmine [OPTIONS]

Options:
  -n, --program-name <name>  Java class name [default: JasmineProgram]
  -r, --skip-rewrite         Just print (or save) the AST
  -i, --input <INPUT>        Input file [default: program.jasmine]
  -s, --save                 Save the file
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```

## Examples


### Hello World
```
fn main() {
	println("Hello, World!");
}
```

### Fibonacci
```
fn fib(n: int) -> int {
	if n == 0 {
		return 0;
	} else if n == 1 {
		return 1;
	} else {
		return fib(n - 1) + fib(n - 2);
	}
}

fn main() {
	println("{}", fib(10));
}
```
