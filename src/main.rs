use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read};

fn load(path: &str) -> Result<File, &str> {
    match File::open(&path) {
        Ok(file) => { Ok(file) }, 
        Err(_e) =>{ Err("load error. file not found.") }
    }
}

fn input(start: usize, words: &Vec<String>) -> String {
    let mut input: String = String::new();
    for (i, _arg) in words.iter().enumerate() {
        match i > start {
            true => { input.push_str(&words[i]); input.push(' '); }
            false => {}
        }
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

fn table(args: &Vec<String>) -> (usize, String, String) {
    if args[1].eq("flex") {
        let file = load(&args[2]).unwrap();
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        (
            2,
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap()
        )
    } else {
        (
            0, 
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>".to_string(),
            "АВСDЕҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdеfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>".to_string()
        )
    }
}

fn stdin() -> Option<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer);
    Some(buffer)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let out = match args.len() {
        1 => { // input from stdin w/o option
            let table = table(&vec![String::new(),String::new()]);
            output(stdin().unwrap(), table)
        },
        3 => { // input from stdin with option
            if args[1].eq("flex") {
                let table = table(&args);
                output(stdin().unwrap(), table)
            } else {
                let table = table(&args);
                let input = input(table.0, &args);
                output(input, table)       
            }
        },
        _ => { // input from args
            let table = table(&args);
            let input = input(table.0, &args);
            output(input, table)
        }
    };
    println!("{}", out);
    Ok(())
}
