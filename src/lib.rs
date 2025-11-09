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

    Some(Controller {
        name,
        class_mapping,
        methods,
    })
}

fn extract_name(text: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();

    for i in 0..words.len() {
        if words[i] == "class"
            && let Some(name) = words.get(i + 1)
        {
            let clean_name = name.trim_end_matches('{').to_string();
            return Some(clean_name);
        }
    }
    None
}

fn extract_mapping(text: &str) -> Option<String> {
    text.find("@RequestMapping")
        .and_then(|i| text[i..].split_once('(')?.1.split_once(')'))
        .map(|(args, _)| args.trim().trim_matches('"').to_string())
}

fn extract_methods(s: &str) -> Vec<ControllerMethod> {
    const ANNOTMETHOD: [&str; 6] = [
        "@GetMapping",
        "@PostMapping",
        "@PutMapping",
        "@DeleteMapping",
        "@PatchMapping",
        "@RequestMapping",
    ];

    let mut methods = Vec::new();

    for annot in ANNOTMETHOD {
        let mut start = 0;
        while let Some(pos) = s[start..].find(annot) {
            let main_pos = start + pos;
            if let Some(m) = extract_method(s, annot, main_pos) {
                methods.push(m);
            }
            start = main_pos + annot.len();
        }
    }

    methods
}

fn extract_method(s: &str, ann: &str, start_idx: usize) -> Option<ControllerMethod> {
    let after = &s[start_idx..];
    let header = after
        .split_once('{')?
        .0
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())?
        .trim()
        .to_string();
    let ann_args = after
        .split_once('(')?
        .1
        .split_once(')')?
        .0
        .trim()
        .trim_matches('"')
        .to_string();

    Some(ControllerMethod {
        annotation: Some(ann.trim_start_matches('@').to_string()),
        annotation_args: Some(ann_args),
        header,
    })
}
