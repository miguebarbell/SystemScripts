mod args;
mod coin;
mod dice;
mod ruleta;

use clap::Parser;

use crate::args::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Coin => {
            let coin = coin::flip_coin();
            println!("{:?}", coin)
        }
        Commands::Dice(number_of_dices) => {
            let dices = dice::run_dices(number_of_dices);
            println!("{:?}", dices)
        }
        Commands::Rulette(options) => {
            let random_option = ruleta::ruleta(options);
            println!("{:?}", random_option)
        }
    }
}
