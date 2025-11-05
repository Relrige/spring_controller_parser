use pest::Parser;
use anyhow::{anyhow, Result};
use spring_controller_parser::*; 

#[test]
fn identifier_test() -> Result<()> {
    let input = "MyTestController";
    let pair = Grammar::parse(Rule::identifier, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}
