use std::collections::HashMap;

use crate::{tokenizer::Token, Json, JsonValue};

pub fn parse(tokens: Vec<Token>) -> Json {
    if matches!(tokens[0], Token::OpenBrace) {
        if matches!(tokens.last().unwrap(), Token::CloseBrace) {
            let (object, _) = parse_object(&tokens, 0);
            return object;
        } else {
            panic!("invalid json")
        }
    } else if matches!(tokens[0], Token::OpenSquareBrace) {
        if matches!(tokens.last().unwrap(), Token::CloseSquareBrace) {
            return parse_array(&tokens[1..tokens.len() - 1]);
        } else {
            panic!("invalid json")
        }
    } else {
        panic!("invalid json")
    }
}

fn parse_object(tokens: &[Token], start_index: usize) -> (Json, usize) {
    let mut object: HashMap<String, JsonValue> = HashMap::new();
    let mut i = start_index + 1;
    let mut end_index = i;
    let mut expecting_comma = false;
    while i < tokens.len() {
        if let Token::CloseBrace = &tokens[i] {
            end_index = i;
            break;
        }
        if expecting_comma {
            if let Token::Comma = tokens[i] {
                expecting_comma = false;
                i += 1;
                continue;
            } else {
                panic!("expected a commma after kv pair");
            }
        }
        if i + 2 < tokens.len() {
            let key: String;
            let value: JsonValue;
            if let Token::String(key_val) = &tokens[i] {
                key = key_val.clone();
            } else {
                panic!("expected a key");
            }

            if !matches!(tokens[i + 1], Token::Colon) {
                panic!("expected a colon after key");
            }

            if let Token::String(val) = &tokens[i + 2] {
                value = JsonValue::String(val.clone());
            } else if let Token::Number(val) = &tokens[i + 2] {
                value = JsonValue::Number(val.clone());
            } else {
                panic!("expected a value, got: {:#?}", tokens[i + 2]);
            }
            object.insert(key, value);
            expecting_comma = true;
            i += 2;
        }
        i += 1;
    }
    return (Json::Object(object), end_index);
}

fn parse_array(tokens: &[Token]) -> Json {
    let mut array = Vec::new();
    let mut i = 0;
    let mut expecting_comma = false;
    while i < tokens.len() {
        if expecting_comma && !matches!(tokens[i], Token::Comma) {
            panic!("was expecting a comma");
        }
        match &tokens[i] {
            Token::String(val) => {
                array.push(JsonValue::String(val.clone()));
                expecting_comma = true;
            }
            Token::Number(val) => {
                array.push(JsonValue::Number(val.clone()));
                expecting_comma = true;
            }
            Token::Comma => {
                expecting_comma = false;
            }
            Token::OpenBrace => {
                let (object, end_index) = parse_object(tokens, i);
                array.push(JsonValue::Json(object));
                i = end_index;
                expecting_comma = true;
            }
            _ => panic!("invalid array"),
        }
        i += 1;
    }
    return Json::Array(array);
}
