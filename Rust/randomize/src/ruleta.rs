use rand::Rng;

pub fn ruleta(options: Vec<&String>) -> &String {
    // options.get(0).unwrap()

    let mut rng = rand::thread_rng();
    options[rng.gen_range(0..options.len())]
    // choose(&mut rand::thread_rng())
}

#[cfg(test)]
mod ruleta_test {
    use crate::ruleta::ruleta;
    #[test]
    fn one_option() {
        let blue: String = "blue".to_string();
        assert_eq!(ruleta([&"blue".to_string()].to_vec()), &blue)
    }

    #[test]
    fn flag_colors() {
        let blue = "blue".to_string();
        let white = "white".to_string();
        let red = "red".to_string();
        let flag_colors: Vec<&String> = [&blue, &white, &red].to_vec();
        let color = ruleta(flag_colors.clone());
        assert!(flag_colors.contains(&color))
    }
}
