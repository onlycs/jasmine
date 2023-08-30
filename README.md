# The Jasmine Programming Language

My computer science class forces me to use the absolutely horrid Java programming language. Because of this,
I decided to make my own programming language that compiles to Java source code. I called it Jasmine

## Features

- [x] Functions
- [x] Loops
- [x] Variables
- [x] Expressions
- [x] Statements
- [x] Structures
- [x] Functions on structures (impls)
- [] Traits (interfaces)
- [x] Enums
- [x] Auto-Expansion of `fn main()` to `public static void main(String[] args)`
- [x] Math
- [x] If-let statements (for enums only)
- [x] Match statements (for enums only)
- [x] Nice builtins
- [] Type casting (not planned)
- [x] Idk what else should be here, but probably, yes!

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
Note: Untested


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