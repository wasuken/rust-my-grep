extern crate regex;
extern crate glob;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::fs::{self, DirEntry};
use std::path::Path;
use regex::Regex;
use glob::glob;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let reg_str = &args[1].parse::<String>().unwrap();
    let filepath = &args[2].parse::<String>().unwrap();

    if Regex::new(r"\W").unwrap().is_match(filepath) {
        for entry in glob(filepath).expect("Faildd to read glob patern"){
            match entry{
                Ok(path) => {
                    let result = file_grep(reg_str, path.to_str().unwrap())
                        .map(|x| format!("{} : {}", path.to_str().unwrap(), x))
                        .fold("".to_string(), |mut result, x| {result.push_str(&x);result.push_str("\n");result});
                    println!("{}", result);
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }else{
        let result = file_grep(reg_str, filepath)
            .fold("".to_string(), |mut result, x| {result.push_str(&x);result.push_str("\n");result});
        println!("{}", result);
    }

}

fn file_grep(reg_str:&str, path: &str) -> Box<Iterator<Item = String>> {
    let re = Regex::new(reg_str).unwrap();
    let reader = BufReader::new(File::open(path).unwrap());

    let result = reader
        .lines()
        .map(|x| x.unwrap())
        .filter(move |x| re.is_match(x));
    Box::new(result)
        // .fold("".to_string(), |mut result, x| {result.push_str(&x);result.push_str("\n");result});
}
