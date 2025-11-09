use pest::{Parser, iterators::Pair};
use thiserror::Error;

pub mod ast;
use ast::{Controller, ControllerMethod};

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum SpringControllerParserError {
    #[error("parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("no controllers found error")]
    NoControllers,
}

pub fn parse_controllers(src: &str) -> Result<Vec<Controller>, SpringControllerParserError> {
    let parse_result = Grammar::parse(Rule::controller_file, src)?;
    let controllers: Vec<_> = parse_result
        .filter_map(|pair| extract_controller(pair))
        .collect();

    if controllers.is_empty() {
        Err(SpringControllerParserError::NoControllers)
    } else {
        Ok(controllers)
    }
}

fn extract_controller(pair: Pair<Rule>) -> Option<Controller> {
    let parsed = pair.as_str();
    if !(parsed.contains("@Controller") || parsed.contains("@RestController")) {
        return None;
    }

    let name = extract_name(parsed)?;
    let class_mapping = extract_mapping(parsed);
    let methods = extract_methods(parsed);

    Some(Controller { name, class_mapping, methods })
}

fn extract_name(text: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();

    for i in 0..words.len() {
        if words[i] == "class" {
            if let Some(name) = words.get(i + 1) {
                let clean_name = name.trim_end_matches('{').to_string();
                return Some(clean_name);
            }
        }
    }
    None
}

fn extract_mapping(text: &str) -> Option<String> {
    if let Some(start) = text.find("@RequestMapping") {
        let after_part = &text[start..];
        let part_before = after_part.split(')').next().unwrap_or("");
        return Some(part_before.to_string());
    }
    None
}

fn extract_methods(s: &str) -> Vec<ControllerMethod> {
    let mut methods = Vec::new();

    methods
}
