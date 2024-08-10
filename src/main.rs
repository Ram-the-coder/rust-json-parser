use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

use json_parser::parse_json;
use json_parser::Json;

use std::time::Instant;

fn main() {
    match read_lines("./src/data/1mb.json") {
        Ok(lines) => {
            let mut json = String::new();
            for line in lines.flatten() {
                json += line.as_str();
            }
            let start = Instant::now();
            let result = parse_json(json);
            println!("duration: {}ms", start.elapsed().as_millis());
            match result {
                Json::Array(array) => {
                    println!("num items: {}", array.len())
                }
                _ => panic!("was expecting array"),
            }
        }
        Err(error) => panic!("{}", error),
    }
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
