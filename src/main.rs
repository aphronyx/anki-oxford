mod cli;

use clap::Parser as _;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
}
