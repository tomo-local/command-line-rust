use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename){
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => println!("{}: OK", filename),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    #[allow(dead_code)]
    files: Vec<String>,
    #[allow(dead_code)]
    number_lines: bool,
    #[allow(dead_code)]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Tomohiro Togashi <tomohiro.togashi@gmail.com>").about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
        ).arg(
            Arg::with_name("number_lines")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank")
        ).arg(
            Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number nonblank lines")
            .takes_value(false)
        ).get_matches();

        Ok(Config {
            files: matches.values_of_lossy("files").unwrap(),
            number_lines: matches.is_present("number_lines"),
            number_nonblank_lines: matches.is_present("number_nonblank"),
        })
}

fn open(filename: &str) ->MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
