pub mod string_util {
  // this fn should be like this -> original.unwrap_or_default()
    pub fn check_string_return_string(original: &Option<String>) -> String {
        match original {
            None => "".to_string(),
            Some(i) => i.to_string(),
        }
    }

    pub fn check_string_return_string_or_none(original: &Option<String>) -> Option<&String> {
        match original {
            None => None,
            Some(i) => Some(i),
        }
    }
}

// &str -> slice - references, primitive
// String -> Vector
