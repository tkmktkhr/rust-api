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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_string_return_string_return_empty_str() {
        let none = None;
        let res = string_util::check_string_return_string(&none);
        assert_eq!(res, "");
    }

    #[test]
    fn check_string_return_string_return_input_str() {
        let str = String::from("str");
        let res = string_util::check_string_return_string(&Some(str));
        assert_eq!(res, "str");
    }

    #[test]
    fn check_string_return_string_or_none_return_none() {
        let none = None;
        let res = string_util::check_string_return_string_or_none(&none);
        assert_eq!(res, None);
    }

    #[test]
    fn check_string_return_string_or_none_return_str() {
        let string = Some(String::from("str"));
        let expected_str = String::from("str");
        let a = Some(&expected_str);
        let res = string_util::check_string_return_string_or_none(&string);
        assert_eq!(res, a);
    }
}
