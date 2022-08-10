use clap::Parser;
use fizzbuzz::cli::Cli;
use fizzbuzz::generator::Generator;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

fn main() {
    let cli: Cli = Cli::from_args();

    let gen = Generator::new(
        Some(cli.max),
        cli.min,
        cli.defines
            .unwrap_or_else(|| vec![("Buzz".to_string(), 5), ("Fizz".to_string(), 3)])
            .into_iter()
            .collect(),
    );

    // SAFETY: 128 bit computers don't exist yet, so this is safe for now
    let bar = ProgressBar::new(cli.max.try_into().expect("Usize too big")).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
            // SAFETY: I have tested multiple times and the template is indeed correct
            .expect("failed to parse template")
            .progress_chars("##-"),
    );

    let value = gen
        .progress_with(bar.clone())
        .map(|val| {
            bar.set_message(val.clone());
            val
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", value);
}
