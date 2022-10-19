mod dice;
mod ruleta;

fn main() {
    println!("Hello, world!");
    println!("{}", dice::run_a_dice());
    println!("{:?}", dice::run_dices(5));
}
