use clap::StructOpt;
use std::{error::Error, str::FromStr};

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

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync>>
where
    T: FromStr,
    T::Err: Error + 'static,
    U: FromStr,
    U::Err: Error + 'static,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: Sync,
    <U as FromStr>::Err: Send,
    <U as FromStr>::Err: Sync,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_val() {
        assert_eq!(
            parse_key_val("fizz=3").unwrap(),
            ("fizz".to_string(), "3".to_string())
        );
    }

    #[test]
    fn test_parse_key_val_error() {
        assert_eq!(
            parse_key_val::<String, i32>("fizz")
                .unwrap_err()
                .to_string(),
            "invalid KEY=value: no `=` found in `fizz`".to_string()
        );
    }

    #[test]
    fn verify_app() {
        use clap::IntoApp;
        Cli::into_app().debug_assert()
    }
}
