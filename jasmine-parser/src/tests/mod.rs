use crate::prelude::*;

#[test]
pub fn test() -> Result<(), ParserError> {
    init_logger().unwrap();

    let test_file = PathBuf::from("../jasmine-parser/src/tests/jasmine/main.jasmine");
    crate::parse(test_file)?;

    Ok(())
}
