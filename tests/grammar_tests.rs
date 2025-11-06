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

#[test]
fn controller_annotation_test() -> Result<()> {
    let cases = ["@Controller", "@RestController"];

    for input in cases {
        let pair = Grammar::parse(Rule::controller_annotation, input)?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_str(), input);
    }

    Ok(())
}

#[test]
fn controller_block_test() -> Result<()> {
    let input = r#"@RestController
        class MyControllerForTest {
            @GetMapping("/hi")
            public String hi() {
                return "Hi";
            }

            @PostMapping("/test")
            public void testData() {
            }
        }
    "#;

    let pair = Grammar::parse(Rule::controller_block, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    
    let text = pair.as_str();
    assert!(text.contains("@RestController"));
    assert!(text.contains("class MyControllerForTest"));
    assert!(text.contains("@GetMapping"));
    assert!(text.contains("hi"));
    Ok(())
}

#[test]
fn controller_file_test() -> Result<()> {
    let input = r#"
        @Controller
        class HelloController {
            @GetMapping("/hi")
            public String hi() {
                return "hi";
            }
        }

        @RestController
        class ApiTestController {
            @PostMapping("/test")
            public void test() {}
        }
    "#;

    let pair = Grammar::parse(Rule::controller_file, input)?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    
    let text = pair.as_str();
    assert!(text.contains("@Controller"));
    assert!(text.contains("HelloController"));
    assert!(text.contains("@RestController"));
    assert!(text.contains("ApiTestController"));
    Ok(())
}