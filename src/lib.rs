//! Spring Controller Parser
//!
//! This crate parses Java Spring controller classes using Pest grammar and produces structured AST data
use pest::{Parser, iterators::Pair};
use thiserror::Error;

pub mod ast;
use ast::{Controller, ControllerMethod};

///The 'Grammar' struct that uses `grammar.pest` to define rules
#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

/// The `SpringControllerParserError` enum represents errors that can haapen during parsing
#[derive(Error, Debug)]
pub enum SpringControllerParserError {
    /// Error returned by Pest parser
    #[error("parsing error: {0}")]
    PestError(#[from] Box<pest::error::Error<Rule>>),
    /// No controllers were found in the source
    #[error("no controllers found error")]
    NoControllers,
    /// Failed to extract method annotation arguments
    #[error("failed to extract annotation arguments for method: {0}")]
    MethodAnnotationError(String),
    #[error("failed to extract class name")]
    /// Could not extract class name
    ClassNameError,
    /// Eextraction errors
    #[error("extraction error: {0}")]
    ExtractionError(String),
}
/// Parses a Java source string and returns all detected Spring controllers
/// # Arguments
/// * `src` – The Java source code as a string slice
///
/// # Returns
/// A `Vec<Controller>` on success, or [`SpringControllerParserError`] on failure
/// # Example
/// ```rust
/// let src = r#"
/// @RestController
/// class TestController {
///     @GetMapping("/test")
///     public String test() { return "test"; }
/// }
/// "#;
///
/// let parsed = spring_controller_parser::parse_controllers(src).unwrap();
/// ```
pub fn parse_controllers(src: &str) -> Result<Vec<Controller>, SpringControllerParserError> {
    let parse_result = Grammar::parse(Rule::controller_file, src)
        .map_err(|e| SpringControllerParserError::PestError(Box::new(e)))?;
    let mut controllers = Vec::new();
    for pair in parse_result {
        controllers.push(extract_controller(pair)?);
    }
    if controllers.is_empty() {
        Err(SpringControllerParserError::NoControllers)
    } else {
        Ok(controllers)
    }
}

/// Extracts a [`Controller`] from the parsed [`Pair`]
///
/// # Arguments
/// * `pair` — A Pest [`Pair<Rule>`] representing a parsed controller block
///
/// # Returns
/// * `Some(Controller)` if the pair represents a Spring controller
/// * `None` if no controller annotation is present
fn extract_controller(pair: Pair<Rule>) -> Result<Controller, SpringControllerParserError> {
    let parsed = pair.as_str();
    if !(parsed.contains("@Controller") || parsed.contains("@RestController")) {
        return Err(SpringControllerParserError::NoControllers);
    }

    let name = extract_name(parsed)?;
    let class_mapping = extract_mapping(parsed)?;
    let methods = extract_methods(parsed)?;

    Ok(Controller {
        name,
        class_mapping,
        methods,
    })
}
/// Extracts the class name from a `class` declaration
/// # Arguments
/// * `text` — The Java src text containing a optionally class declaration
///
/// # Returns
/// * `Some(String)` containing the class name
/// * `None` if no class name is found
fn extract_name(text: &str) -> Result<String, SpringControllerParserError> {
    let words: Vec<&str> = text.split_whitespace().collect();

    for i in 0..words.len() {
        if words[i] == "class"
            && let Some(name) = words.get(i + 1)
        {
            return Ok(name.trim_end_matches('{').to_string());
        }
    }

    Err(SpringControllerParserError::ClassNameError)
}
/// Extracts the argument of a class-level `@RequestMapping` annotation
/// # Arguments
/// * `text` — The full Java src of the controller class
///
/// # Returns
/// * `Some(String)` with the request mapping path
/// * or `None` if no `@RequestMapping` is found
fn extract_mapping(text: &str) -> Result<Option<String>, SpringControllerParserError> {
    if let Some(idx) = text.find("@RequestMapping") {
        let args = text[idx..]
            .split_once('(')
            .and_then(|(_, rem)| rem.split_once(')'))
            .ok_or_else(|| {
                SpringControllerParserError::ExtractionError("@RequestMapping args".into())
            })?
            .0
            .trim()
            .trim_matches('"')
            .to_string();
        Ok(Some(args))
    } else {
        Ok(None)
    }
}
/// Extracts all controller methods with mapping annotations
/// Recognizes:
/// - `@GetMapping`
/// - `@PostMapping`
/// - `@PutMapping`
/// - `@DeleteMapping`
/// - `@PatchMapping`
/// - `@RequestMapping`
///
/// # Arguments
/// * `s` — The Java src code of the controller class
///
/// # Returns
/// * A `Vec<ControllerMethod>` representing all detected methods with mappings
fn extract_methods(s: &str) -> Result<Vec<ControllerMethod>, SpringControllerParserError> {
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
            methods.push(extract_method(s, annot, main_pos)?);
            start = main_pos + annot.len();
        }
    }

    Ok(methods)
}
/// Extracts a single controller method starting at the given position.
/// Returns a [`ControllerMethod`] with the annotation, arguments, and method header.
/// # Arguments
/// * `s` — The full Java controller src text
/// * `ann` — The mapping annotation being processed
/// * `start_idx` — The starting index of the annotation within `s`
///
/// # Returns
/// * `Some(ControllerMethod)` if a valid annotated method is found
/// * `None` if parsing fails due to bad syntax
///
fn extract_method(
    s: &str,
    ann: &str,
    start_idx: usize,
) -> Result<ControllerMethod, SpringControllerParserError> {
    let after = &s[start_idx..];

    let header = after
        .split_once('{')
        .ok_or_else(|| SpringControllerParserError::MethodAnnotationError(ann.into()))?
        .0
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .ok_or_else(|| SpringControllerParserError::MethodAnnotationError(ann.into()))?
        .trim()
        .to_string();

    let ann_args = after
        .split_once('(')
        .and_then(|(_, rem)| rem.split_once(')'))
        .map(|(args, _)| args.trim().trim_matches('"').to_string())
        .ok_or_else(|| SpringControllerParserError::MethodAnnotationError(ann.into()))?;

    Ok(ControllerMethod {
        annotation: Some(ann.trim_start_matches('@').to_string()),
        annotation_args: Some(ann_args),
        header,
    })
}
