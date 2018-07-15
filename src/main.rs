use std::env::args;
use std::fs::{File};
use std::io::{Read};

pub fn load(path: &str) -> Result<String, &str> {
    println!("Attempting to load page: {:?}", path);
    let mut buf = String::new();
    match File::open(&path) {
        Ok(mut file) => { 
            file.read_to_string(&mut buf).unwrap();
            Ok(buf)
        }, 
        Err(_e) =>{ Err("load error. file not found.") }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: String = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            input.push_str(&args[i]);
            input.push(' ');
        }
    }

    let latin: &str =  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890!\"§$%&/()=?,.-;:_";
    let cyrill: &str = "АВСDЕҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdеfgніјкlмпорqгѕтцѵшхчz 1234567890!\"§$%&/()=?,.-;:_";
    let cvec: Vec<_> = cyrill.chars().collect();
    let mut output: String = String::new();
    for c in input.chars() {
        for (i, cl) in latin.chars().enumerate() {
             if c.eq(&cl) { output.push(cvec[i]); }
             else { }
        }
    }
    println!("{}", output);
}
