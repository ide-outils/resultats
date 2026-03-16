pub use resultats::Result::{self, Err, Ok};

fn parse(s: &str) -> Result<String> {
    let value = s
        .parse::<i32>()
        .map_err(|err| format!("{err} ('{s}')"))?;
    Ok(value.to_string())
}

fn parse_nums() -> Result<()> {
    let iter = vec!["1", "2", "trois", "4", "..."].into_iter();
    let iter2 = vec!["1", "2", "4", "cinq"].into_iter();
    let parsed: Result<Vec<_>> = iter.map(parse).chain(iter2.map(parse)).collect();
    println!("{}", parsed?.join(" ; "));
    Ok(())
}

fn main() -> Result<()> {
    parse_nums()
}
