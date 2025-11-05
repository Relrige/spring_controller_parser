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


#[test]
fn class_decl_test() -> Result<()> {
    let input = "class MyTestController";
    let pair = Grammar::parse(Rule::class_decl, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn method_test() -> Result<()> {
    let input ="public String hello() { return \"hi\"; }";
    let pair = Grammar::parse(Rule::method, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert!(pair.as_str().contains("hello"));
    Ok(())
}

#[test]
fn mapping_args_test() -> Result<()> {
    let input = "(\"/api/test\")";
    let pair = Grammar::parse(Rule::mapping_args, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn mapping_annotation_test() -> Result<()> {
    let cases = [
        "@RequestMapping(\"/api\")",
        "@GetMapping(\"/get\")",
        "@PostMapping(\"/post\")",
        "@DeleteMapping(\"/delete\")",
        "@PatchMapping(\"/patch\")",
        "@PutMapping(\"/put\")",
    ];

    for input in cases {
        let pair = Grammar::parse(Rule::mapping_annotation, input)?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), input);
    }

    Ok(())
}