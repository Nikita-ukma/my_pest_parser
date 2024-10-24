use pest_derive::Parser;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {
   let got: Pairs<'_, Rule> = Grammar::parse(Rule::file, "-273.15,-15\n")?;
    println!("{:?}", got);
   Ok(())
}
