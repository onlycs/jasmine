const TEST_INPUT: &'static str = "
type K = OtherType;

struct S {
	a: K,
	b: Vec<K>,
}

struct N(K);

fn f<L: K>(a: K, b:L) -> Z {}
fn f<L: K>(a: K, b:L) -> Z;
";

fn main() {
    match parser::parse(TEST_INPUT) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
