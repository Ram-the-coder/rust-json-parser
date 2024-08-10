#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    String(String),
    Colon,
    Number(String),
    Comma,
    OpenSquareBrace,
    CloseSquareBrace,
}

const DIGITS: &str = "0123456789.";

pub fn tokenize(json: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut chars = json.chars().peekable();

    loop {
        let ch_opt = chars.next();
        if ch_opt.is_none() {
            break;
        }
        let ch = ch_opt.unwrap();
        // println!("processing char: {}", ch);
        if ch == '{' {
            result.push(Token::OpenBrace);
            continue;
        } else if ch == '}' {
            result.push(Token::CloseBrace);
            continue;
        } else if ch == '[' {
            result.push(Token::OpenSquareBrace);
            continue;
        } else if ch == ']' {
            result.push(Token::CloseSquareBrace);
            continue;
        } else if ch == ':' {
            result.push(Token::Colon);
            continue;
        } else if ch == ',' {
            result.push(Token::Comma);
            continue;
        } else if ch == '"' {
            let mut next_ch = chars.next().expect("matching double quote not found");
            let mut value = String::new();
            while next_ch != '"' {
                value.push(next_ch);
                next_ch = chars.next().expect("matching double quote not found");
            }
            result.push(Token::String(value));
            continue;
        } else if DIGITS.contains(ch) {
            let mut value: String = ch.to_string();
            loop {
                let next_ch_opt = chars.peek();
                if next_ch_opt.is_none() {
                    break;
                }
                if let Some(next_ch) = next_ch_opt {
                    if DIGITS.contains(*next_ch) {
                        value.push(*next_ch);
                        chars.next(); // consume next digit
                    } else {
                        break;
                    }
                }
            }
            result.push(Token::Number(value));
        }
    }
    result
}
