use pest_01::{MyParser, Record}; // Assuming `pest_01` is the crate name
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() -> anyhow::Result<()> {
        let input = "";
        let result = MyParser::parse_file(input)?;
        let expected: Vec<Record> = vec![];
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_incorrect_format() {
        let input = "123,456,789,";
        let result = MyParser::parse_file(input);
        assert!(result.is_err(), "Expected an error due to incorrect format");
    }
}