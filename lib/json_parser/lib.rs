use std::collections::HashMap;

mod parser;
mod tokenizer;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(String),
    Json(Json),
}

#[derive(Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

pub fn parse_json(json: String) -> Json {
    let tokens = tokenizer::tokenize(&json);
    // println!("Tokenizer output");
    // println!("----------------");
    // for token in &tokens {
    //     println!("{:?}", token)
    // }
    // println!("----------------");
    if tokens.len() == 0 {
        panic!("invalid json")
    }
    let result = parser::parse(tokens);
    return result;
}
