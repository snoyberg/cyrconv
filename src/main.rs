use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read};

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

fn output(input: String, table: (usize, String, String) ) -> String {
    let tvec: Vec<_> = table.2.chars().collect();
    let mut output: String = String::new();
    for c in input.chars() {
        for (i, cl) in table.1.chars().enumerate() {
            match c.eq(&cl) {
                true => { output.push(tvec[i]); },
                false => {}
            }
        }
    }
    output
}

fn default() -> (usize, String, String) {
    (
        1,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>^".to_string(),
        "ДВСDЁҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdёfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>ˇ".to_string()
    )
}

fn table(args: &Vec<String>) -> Result<(usize, String, String), String> {
    if args.len() > 1 && args[1] == "flex" {
        let file = load(&args[2])?;
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        let line1 = pop_line(&mut lines)?;
        let line2 = pop_line(&mut lines)?;
        Ok((2, line1, line2))
    } else {
        Ok(default())
    }
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

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let table = table(&args)?;
    let out = match args.len() {
        1 => { // input from stdin w/o option
            output(stdin()?, table)
        },
        3 => { // input from stdin with option
            if args[1].eq("flex") {  output(stdin()?, table) }
            else {
                let input = input(&args[table.0 ..]);
                output(input, table)
            }
        },
        _ => { // input from args
            let input = input(&args[table.0..]);
            output(input, table)
        }
    };
    println!("{}", out);
    Ok(())
}
