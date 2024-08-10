#[cfg(test)]
mod tests {
    use json_parser::parse_json;
    use json_parser::Json;
    use json_parser::JsonValue;

    #[test]
    fn test_should_panic_for_empty_string() {
        let result = std::panic::catch_unwind(|| parse_json(String::from("")));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_return_empty_json_object() {
        parse_json(String::from("{}"));
    }

    #[test]
    fn test_should_return_json_object_with_string_value() {
        let json = parse_json(String::from(r#"{"key1": "value1"}"#));
        match json {
            Json::Object(object) => {
                assert_eq!(
                    object.get("key1"),
                    Some(&JsonValue::String(String::from("value1")))
                );
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_should_return_json_object_with_number_value() {
        let json = parse_json(String::from(r#"{"key1": 123}"#));
        match json {
            Json::Object(object) => {
                assert_eq!(
                    object.get("key1"),
                    Some(&JsonValue::Number(String::from("123")))
                );
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_should_return_json_object_with_number_value_with_decimal() {
        let json = parse_json(String::from(r#"{"key1": 1.23}"#));
        match json {
            Json::Object(object) => {
                assert_eq!(
                    object.get("key1"),
                    Some(&JsonValue::Number(String::from("1.23")))
                );
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_should_return_json_object_with_multiple_kv_pairs() {
        let json = parse_json(String::from(r#"{"key1": 123, "key2": "value2"}"#));
        match json {
            Json::Object(object) => {
                assert_eq!(
                    object.get("key1"),
                    Some(&JsonValue::Number(String::from("123")))
                );
                assert_eq!(
                    object.get("key2"),
                    Some(&JsonValue::String(String::from("value2")))
                );
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_should_return_empty_array() {
        let json = parse_json(String::from(r#"[]"#));
        if let Json::Array(array) = json {
            assert_eq!(array.len(), 0)
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_should_return_array_with_string_values() {
        let json = parse_json(String::from(r#"["one", "two"]"#));
        if let Json::Array(array) = json {
            assert_eq!(array.len(), 2);
            assert_eq!(array[0], JsonValue::String(String::from("one")));
            assert_eq!(array[1], JsonValue::String(String::from("two")));
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_should_return_array_with_number_values() {
        let json = parse_json(String::from(r#"[1, 2]"#));
        if let Json::Array(array) = json {
            assert_eq!(array.len(), 2);
            assert_eq!(array[0], JsonValue::Number(String::from("1")));
            assert_eq!(array[1], JsonValue::Number(String::from("2")));
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_should_return_array_of_objects() {
        let json = parse_json(String::from(r#"[{"k11": "v11"}, {"k21": "v21"}]"#));
        if let Json::Array(array) = json {
            assert_eq!(array.len(), 2);
            match &array[0] {
                JsonValue::Json(json) => {
                    if let Json::Object(object) = json {
                        assert_eq!(
                            object.get("k11"),
                            Some(&JsonValue::String(String::from("v11")))
                        )
                    } else {
                        panic!("expected object");
                    }
                }
                _ => panic!("expected object"),
            }
        } else {
            panic!("expected array");
        }
    }
}
