use std::error::Error;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// minimum value as an integar
    pub min: u64,
    /// minimum value as an integar
    pub max: u64,
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    /// create custom matchers
    ///
    /// syntax: replace=number
    ///
    /// example: fizzbuzz 0 10 -d fizz=3 buzz=5 bazz=7
    pub defines: Option<Vec<(String, u64)>>,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
