use rand::Rng;

use crate::args::RuletteOptions;

pub fn ruleta(RuletteOptions { options }: &RuletteOptions) -> String {
    let mut rng = rand::thread_rng();
    let split_options: Vec<String> = options
        .split(',')
        .map(|x| x.to_string())
        // .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let random_index = rng.gen_range(0..split_options.len());
    split_options.get(random_index).unwrap().trim().to_string()
}

#[cfg(test)]
mod ruleta_test {
    use crate::args::RuletteOptions;
    use crate::ruleta::ruleta;
    #[test]
    fn one_option() {
        let blue: RuletteOptions = RuletteOptions {
            options: "blue".to_string(),
        };

        assert_eq!(ruleta(&blue), blue.options)
    }

    #[test]
    fn flag_colors() {
        let flag_colors: RuletteOptions = RuletteOptions {
            options: "blue,white,red".to_string(),
        };
        let color = ruleta(&flag_colors);
        assert!(flag_colors.options.contains(&color))
    }
}
