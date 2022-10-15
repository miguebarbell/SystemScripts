use clap::Parser;

/// Simple program to convert microsoft calendar tz mess to unix timezone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the ics file.
    #[arg(short, long)]
    file: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
