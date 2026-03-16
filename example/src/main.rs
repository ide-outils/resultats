pub use resultats::Result::{self, Err, Ok};

fn parse(s: &str) -> Result<String> {
    let value = s
        .parse::<i32>()
        .map_err(|err| format!("{err} ('{s}')"))?;
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
