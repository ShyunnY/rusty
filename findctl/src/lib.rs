use std::error::Error;
use std::io::Write;
use std::{env, fs};

use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for ele in search(&config.query, &content, config.is_sensitive) {
        write_color("result", ele, Color::Yellow);
    }
    Ok(())
}

pub fn write_color(prefix: &str, msg: &str, color: Color) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();

    write!(&mut stdout, "{prefix}: ").unwrap();
    stdout.reset().unwrap();
    println!("{msg}");
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    is_sensitive: bool,
}

impl Config {
    const IS_SENSITIVE: &str = "SENSITIVE";

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let is_sensitive = match env::var(Self::IS_SENSITIVE) {
            Ok(v) if !v.eq("0") => true,
            _ => false,
        };

        let query = match args.next() {
            Some(v) => v,
            None => Err("can't to get query")?,
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => Err("can't to get target file")?,
        };

        Ok(Self {
            query,
            file_path,
            is_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str, is_sensitive: bool) -> Vec<&'a str> {
    content
        .lines()
        .into_iter()
        .filter(|&x| {
            let x = match is_sensitive {
                true => x.to_string(),
                false => x.to_lowercase(),
            };
            x.contains(query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let content = "\
xxpp
xxoo
ooxx
        ";

        assert_eq!(
            vec!["xxoo"],
            search("xxoo", content, false),
            "search case-insensitively"
        );
        assert_eq!(
            Vec::<&'static str>::new(),
            search("XxOo", content, true),
            "Search case-sensitively"
        );
    }
}
