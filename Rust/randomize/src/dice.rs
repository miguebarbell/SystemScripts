use rand::Rng;
pub fn run_a_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub fn run_dices(quantity: u8) -> Vec<u8> {
    let dices: Vec<u8> = Vec::with_capacity(quantity.into());
    dices.iter().map(|_| run_a_dice()).collect()
    // for i in 0..dices.len() {
    //     dices[i] = run_a_dice();
    // }
    // dices
}

#[cfg(test)]
mod dice_test {
    use crate::dice::run_a_dice;
    use crate::dice::run_dices;
    #[test]
    fn roll_a_dice() {
        let dice_result = run_a_dice();
        let dice_options: [u8; 6] = [1, 2, 3, 4, 5, 6];
        assert!(dice_options.contains(&dice_result));
    }
    #[test]
    fn get_3_dices() {
        let dices = run_dices(3);
        let dice_options: [u8; 6] = [1, 2, 3, 4, 5, 6];
        dices.iter().for_each(|x| assert!(dice_options.contains(x)))
    }
}
