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

fn main() {
    let mut start = 0;    
    
    let args: Vec<String> = std::env::args().collect();
    let table = if args[1].eq("flex") {
        start = 2;
        load_table(&args[2])
    } else {
        Ok (( 
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_'".to_string(),
            "АВСDЕҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdеfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_'".to_string()
        ))
    }.unwrap();
    let input = input(start, &args);

    let tvec: Vec<_> = table.1.chars().collect();
    let mut output: String = String::new();
    for c in input.chars() {
        for (i, cl) in table.0.chars().enumerate() {
             if c.eq(&cl) { output.push(tvec[i]); }
             else { }
        }
    }
    println!("{}", output);
}
