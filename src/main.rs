use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read};
use std::collections::HashMap;

fn input(words: &[String]) -> String {
    let mut input: String = String::new();
    for word in words {
        input.push_str(word);
        input.push(' ');
    }
    input
}

struct Mapper(HashMap<char, char>);

impl Mapper {
    fn new(x: &str, y: &str) -> Self {
        Mapper(x.chars().zip(y.chars()).collect())
    }

    fn map(&self, c: char) -> char {
        *self.0.get(&c).unwrap_or(&c)
    }

    fn default() -> Self {
        Self::new
            ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>^",
             "ДВСDЁҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdёfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>ˇ"
            )
    }
}

fn output(input: &str, mapper: &Mapper) -> String {
    input.chars().map(|c| mapper.map(c)).collect()
}

fn load_mapper(filepath: &str)-> Result<Mapper, Box<std::error::Error>> {
    let file = File::open(&filepath)?;
    let reader = BufReader::new(&file);
    let mut lines = reader.lines();
    let line1 = pop_line(&mut lines)?;
    let line2 = pop_line(&mut lines)?;
    Ok(Mapper::new(&line1, &line2))
}

fn pop_line<I>(lines: &mut I) -> Result<String, String>
    where I: Iterator<Item=std::io::Result<String>> {
    match lines.next() {
        Some(Err(e)) => Err(e.to_string()),
        Some(Ok(line)) => Ok(line),
        None => Err("not enough lines in flex file".to_string()),
    }
}

fn stdin() -> std::io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

struct Args {
    flex_file: Option<String>,
    words: Vec<String>,
}

fn parse_args<I>(mut iter: I) -> Result<Args, &'static str>
    where I: Iterator<Item=String> {
    iter.next(); // drop the program name
    match iter.next() {
        None => Ok(Args {
            flex_file: None,
            words: vec![],
        }),
        Some(first) => {
            if first == "flex" {
                match iter.next() {
                    None => Err("flex requires one argument"),
                    Some(file) => {
                        Ok(Args {
                            flex_file: Some(file),
                            words: iter.collect(),
                        })
                    }
                }
            } else {
                Ok(Args {
                    flex_file: None,
                    words: std::iter::once(first).chain(iter).collect(),
                })
            }
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args = parse_args(std::env::args())?;
    let mapper = match args.flex_file {
        None => Mapper::default(),
        Some(flex_file) => load_mapper(&flex_file)?,
    };

    let input =
        if args.words.is_empty() {
            stdin()?
        } else {
            input(&args.words)
        }
    ;
    println!("{}", output(&input, &mapper));
    Ok(())
}
