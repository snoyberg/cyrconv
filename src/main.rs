use std::fs::{File};
use std::io::{BufRead, BufReader};

pub fn load_table(path: &str) -> Result<(String, String), &str> {
    match File::open(&path) {
        Ok(file) => { 
            let reader = BufReader::new(&file);
            let mut lines = reader.lines();
            Ok ( ( lines.next().unwrap().unwrap(), lines.next().unwrap().unwrap() ) )
        }, 
        Err(_e) =>{ Err("load error. file not found.") }
    }
}

fn input(start: usize, words: &Vec<String>) -> String {
    let mut input: String = String::new();
    for (i, _arg) in words.iter().enumerate() {
        if i > start {
            input.push_str(&words[i]);
            input.push(' ');
        }
    }
    input
}

fn table(args: &Vec<String>) -> Option<(usize, String, String)> {
    if args[1].eq("flex") {
        let table = load_table(&args[2]).unwrap();
        Some ( (2, table.0, table.1) )
    } else {
        Some ( 
            (
                0, 
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>".to_string(),
                "АВСDЕҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdеfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'{[]}<>".to_string()
            )
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
   
    let table = table(&args).unwrap();
    let input = input(table.0, &args);

    let tvec: Vec<_> = table.2.chars().collect();
    let mut output: String = String::new();
    for c in input.chars() {
        for (i, cl) in table.1.chars().enumerate() {
             if c.eq(&cl) { output.push(tvec[i]); }
             else { }
        }
    }
    println!("{}", output);
}
