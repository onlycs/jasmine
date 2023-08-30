# The Jasmine Programming Language

My computer science class forces me to use the absolutely horrid Java programming language. Because of this,
I decided to make my own programming language that compiles to Java source code. I called it Jasmine

## Features

- [x] Pretty much everything
- [] except for indexing arrays

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