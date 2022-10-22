use crate::args::RuletteOptions;
use crate::ruleta;
pub fn flip_coin() -> String {
    let coin_options: RuletteOptions = RuletteOptions {
        options: "heads, tails".to_string(),
    };
    ruleta::ruleta(&coin_options)
}

#[cfg(test)]
mod coin_test {
    use crate::coin::flip_coin;

    #[test]

    fn flip_coin_test() {
        let heads = "heads".to_string();
        let tails = "tails".to_string();
        let coin_options: Vec<String> = [heads, tails].to_vec();
        let result = flip_coin();
        assert!(coin_options.contains(&result))
    }
}
