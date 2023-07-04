#![allow(unused)]
#![allow(non_snake_case)]

pub mod common;
pub mod schemas;
pub mod package;
pub mod zip;
pub mod abstraction;


#[derive(Debug)]
pub enum PresentationError {
    ReadError(String),
    ConvertError(String)
}


#[cfg(test)]
mod tests {
}
