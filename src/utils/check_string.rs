pub fn check_string(original: &Option<String>) -> std::string::String {
    match original {
        None => "".to_string(),
        Some(i) => i.to_string(),
    }
}

// &str -> slice - references, primitive
// String -> Vector

// pub fn number(x: &Option<u32>) -> std::string::String {
//   match x {
//       None => "".to_string(),
//       Some(i) => i.to_string(),
//   }
// }
