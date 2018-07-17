use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read};
use std::collections::HashMap;

fn load(path: &str) -> Result<File, &str> {
    match File::open(path) {
        Ok(file) => { Ok(file) },
        Err(_e) =>{ Err("load error. file not found.") }
    }
}

fn input(words: &[String]) -> String {
    let mut input: String = String::new();
    for word in words {
        input.push_str(word);
        input.push(' ');
    }
    input
}

fn output(input: String, table: (String, String) ) -> String {
    let hm: HashMap<char, char> =
        table.0.chars()
        .zip(table.1.chars())
        .collect()
        ;
    input.chars().map(|c| hm.get(&c).unwrap_or(&c).clone()).collect()
}

fn default() -> (String, String) {
    (
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>^".to_string(),
        "ДВСDЁҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdёfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>ˇ".to_string()
    )
}

fn load_table(filepath: String)-> Result<(String, String), String> {
    let file = load(&filepath)?;
    let reader = BufReader::new(&file);
    let mut lines = reader.lines();
    let line1 = pop_line(&mut lines)?;
    let line2 = pop_line(&mut lines)?;
    Ok((line1, line2))
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
    let table = match args.flex_file {
        None => default(),
        Some(flex_file) => load_table(flex_file)?,
    };

    let input =
        if args.words.len() == 0 {
            stdin()?
        } else {
            input(&args.words)
        }
    ;
    println!("{}", output(input, table));
    Ok(())
}
