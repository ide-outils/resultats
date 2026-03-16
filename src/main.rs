#![feature(try_trait_v2)]

pub mod ast;
pub mod format_error;
pub mod result;
pub mod source_code;

pub use result::{
    Error, Errors,
    Result::{self, Err, Ok},
};

fn parse(s: &str) -> Result<String> {
    let value = s.parse::<i32>()?;
    Ok(value.to_string())
}

fn ideal() -> Result<()> {
    let iter = vec!["1", "2", "trois", "4", "dsdf", "dsfds", "sd"].into_iter();
    let parsed: Vec<String> = iter.map(parse).collect::<Result<Vec<String>>>()?;
    println!("{}", parsed.join(" ; "));
    Ok(())
}

fn main() {
    if let Err(e) = ideal() {
        eprintln!("{:?}", e);
    }
}
