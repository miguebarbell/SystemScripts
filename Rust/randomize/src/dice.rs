use rand::Rng;

use crate::args::DiceOptions;
pub fn run_a_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub fn run_dices(DiceOptions { number: dices }: &DiceOptions) -> Vec<u8> {
    let mut dices_result: Vec<u8> = Vec::with_capacity(usize::from(*dices));
    for _ in 0..*dices {
        dices_result.push(run_a_dice());
    }
    dices_result
}

#[cfg(test)]
mod dice_test {
    use crate::args::DiceOptions;
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
        let dice_options: DiceOptions = DiceOptions { number: (3) };
        let dices = run_dices(&dice_options);
        let dice_options: [u8; 6] = [1, 2, 3, 4, 5, 6];
        dices.iter().for_each(|x| assert!(dice_options.contains(x)))
    }
}
