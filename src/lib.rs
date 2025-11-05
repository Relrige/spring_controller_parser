use pest::{Parser};
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum SpringControllerError {
    
}



