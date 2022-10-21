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
    /// Optionally provide color stops to control
    /// the gradient's generation
    Coin,
    /// Throw a defined quantity of dices, (default 1)
    Dice(DiceOptions),
    /// Return a random answer from the options provided
    Rulette(RuletteOptions),
}

#[derive(Args, Debug)]
pub struct DiceOptions {
    #[arg(short = 'n', long = "number", default_value_t = 1)]
    pub number: u8,
}

#[derive(Args, Debug)]
pub struct RuletteOptions {
    #[arg(short = 'o', long = "options")]
    pub options: String,
}
