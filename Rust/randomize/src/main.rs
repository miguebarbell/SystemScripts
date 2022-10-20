mod coin;
mod dice;
mod ruleta;

fn main() {
    println!("{}", dice::run_a_dice());
    println!("{:?}", dice::run_dices(5));
    println!(
        "{:?}",
        ruleta::ruleta(
            [
                &"blue".to_string(),
                &"red".to_string(),
                &"white".to_string()
            ]
            .to_vec()
        )
    );
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
    println!("{}", dice::run_a_dice());
    println!("{:?}", dice::run_dices(5));
    println!(
        "{:?}",
        ruleta::ruleta(
            [
                &"blue".to_string(),
                &"red".to_string(),
                &"white".to_string()
            ]
            .to_vec()
        )
    );
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
    println!("{:?}", coin::flip_coin());
}
