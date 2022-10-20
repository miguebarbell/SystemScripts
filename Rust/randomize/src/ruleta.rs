use rand::Rng;

use crate::args::RuletteOptions;

pub fn ruleta(RuletteOptions { options }: &RuletteOptions) -> &String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..options.len());
    options.get(random_index).unwrap()
}

#[cfg(test)]
mod ruleta_test {
    use crate::args::RuletteOptions;
    use crate::ruleta::ruleta;
    #[test]
    fn one_option() {
        let blue: RuletteOptions = RuletteOptions {
            options: ["blue".to_string()].to_vec(),
        };

        assert_eq!(ruleta(&blue), &blue.options[0])
    }

    #[test]
    fn flag_colors() {
        let blue = "blue".to_string();
        let white = "white".to_string();
        let red = "red".to_string();
        let flag_colors: RuletteOptions = RuletteOptions {
            options: [blue, white, red].to_vec(),
        };
        let color = ruleta(&flag_colors);
        assert!(flag_colors.options.contains(color))
    }
}
