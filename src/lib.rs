use pest::Parser;
use pest_derive::Parser;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "grammar.pest"] // refers to the grammar.pest file
pub struct MyParser;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub fields: Vec<String>,
}

impl MyParser {
    pub fn parse_file(input: &str) -> Result<Vec<Record>> {
        let pairs = MyParser::parse(Rule::file, input)?;
        let mut records = Vec::new();

        for pair in pairs {
            if let Rule::record = pair.as_rule() {
                let fields: Vec<String> = pair
                    .into_inner()
                    .filter(|inner_pair| inner_pair.as_rule() == Rule::field)
                    .map(|inner_pair| inner_pair.as_str().to_string())
                    .collect();
                records.push(Record { fields });
            }
        }
        Ok(records)
    }
}