const TEST_INPUT: &'static str = "
type T = K;
type K = __ext_java_int;
";

fn main() {
    match parser::parse(TEST_INPUT) {
        Ok(program) => {
            println!("{:?}", program);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
