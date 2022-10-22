use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Flip a coin
    ///
    /// Flip a coin, the result can be "heads" or "tails"
    Coin,
    /// Throw a defined quantity of dices, (default 1)
    ///
    /// Throw a defined quantity of dices, (default 1), and returns the result in form of array,
    /// ie: [1,5,3] that means that you throw 3 dices, the first result is 1, the second 5, and the
    /// third 3.
    /// dice -n 3
    Dice(DiceOptions),
    /// Return a random answer from the options provided
    ///
    /// Return a random answer from the options provided.
    Rulette(RuletteOptions),
}

#[derive(Args, Debug)]
pub struct DiceOptions {
    /// Number of dices to Throw, default = 1
    #[arg(short = 'n', long = "number", default_value_t = 1)]
    pub number: u8,
}

#[derive(Args, Debug)]
pub struct RuletteOptions {
    /// Options to pick separated by comma (,) and surrounded by double quotes (""), ie: "red,
    /// blue, white"
    #[arg(short = 'o', long = "options")]
    pub options: String,
}
