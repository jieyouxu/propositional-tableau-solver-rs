use std::path::PathBuf;

use colored::*;
use log::info;
use paw;

pub mod logger;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Arguments to the program.
#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Args {
    /// Toggle debug mode.
    ///
    /// This changes the log level to `trace`, so is equivalent to manually setting the environment
    /// variable `LOG=trace`.
    #[structopt(short = "d", long)]
    debug: bool,

    /// Path to input file. (OPTIONAL)
    ///
    /// If the `<input_file>` is specified then `stdin` is ignored.
    /// Otherwise `stdin` input stream is used instead. The program can handle multiple
    /// propositional formulas that are delimited by the newline character (`\n`).
    #[structopt(short = "i", long = "input")]
    input_file: Option<PathBuf>,

    /// Path to output file. (OPTIONAL)
    ///
    /// If the `<output_file>` is not specified then output of the program is written to `stdout`.
    #[structopt(short = "o", long = "output")]
    output_file: Option<PathBuf>,
}

#[paw::main]
pub fn main(args: Args) -> std::io::Result<()> {
    logger::setup(args.debug);

    info!(
        "{}: v{}",
        "Propositional Tableau Solver".cyan().bold().underline(),
        VERSION.unwrap_or("unknown version").yellow()
    );

    info!("arguments provided\n {:#?}", &args);

    Ok(())
}
