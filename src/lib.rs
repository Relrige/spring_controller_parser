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
   None
}