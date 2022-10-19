pub fn ruleta(options: Vec<&String>) -> &String {
    options.get(0).unwrap()
}

#[cfg(test)]
mod ruleta_test {
    use crate::ruleta::ruleta;
    #[test]
    fn one_option() {
        let blue: String = "blue".to_string();
        assert_eq!(ruleta([&"blue".to_string()].to_vec()), &blue)
    }
}
