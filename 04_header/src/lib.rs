use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("header")
        .version("0.1.0")
        .author("Tomohiro Togashi <tomohiro.togashi@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
        ).arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .help("Number of lines")
            .takes_value(true)
            .default_value("10"),
        ).arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .help("Number of bytes")
            .takes_value(true),
        ).get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of("lines").unwrap().parse().unwrap(),
        bytes: matches.value_of("bytes").map(|x| x.parse().unwrap()),
    })
}
