mod cli;
mod generator;

use std::convert::TryInto;

use cli::Cli;
use generator::Generator;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use structopt::StructOpt;

fn main() {
    let cli: Cli = Cli::from_args();

    let gen = Generator {
        match_against: cli
            .defines
            .unwrap_or(vec![("Buzz".to_string(), 5), ("Fizz".to_string(), 3)]),
        max: cli.max,
        current: cli.min,
    };

    let mut value = String::new();
    let bar = ProgressBar::new(cli.max.try_into().unwrap()).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    for val in gen.progress_with(bar.clone()) {
        bar.set_message(&*val);
        value += &format!("{}\n", val);
    }
    println!("{}", value);
}
