use pest::{Parser};
use thiserror::Error;

pub mod ast;
use ast::{Controller, ControllerMethod};

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum SpringControllerError {
    #[error("parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("no controllers found error")]
    NoControllers,
}



