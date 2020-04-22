use std::path::PathBuf;

use colored::*;
use log::{debug, error, info};
use paw;
use std::convert::TryFrom;
use std::fs;
use std::io::{self, prelude::*};

use libprop_sat_solver::formula::PropositionalFormula;
use libprop_sat_solver::tableaux_solver::{is_satisfiable, is_valid};

pub mod logger;
pub mod parser;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Arguments to the program.
#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Args {
    /// Debug output.
    #[structopt(short = "d", long)]
    debug: bool,

    /// A single propositional formula to compute the satisfiability/validity for. (OPTIONAL)
    #[structopt(short = "c", long = "formula")]
    single_formula: Option<String>,

    /// The mode to run the CLI in:
    ///
    /// - `"s"` - output satisfiability of the given formula(s).
    /// - `"v"` - output validity of the given formula(s).
    #[structopt(short = "m", long)]
    mode: Option<char>,

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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub enum CliOutputMode {
    Satisfiability,
    Validity,
}

impl TryFrom<char> for CliOutputMode {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            's' => Ok(Self::Satisfiability),
            'v' => Ok(Self::Validity),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for CliOutputMode {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_ref() {
            "sat" | "satisfiability" => Ok(Self::Satisfiability),
            "val" | "validity" => Ok(Self::Validity),
            _ => Err(()),
        }
    }
}

/// Main entry point to the propositional satisfiability solver.
///
/// # Errors
///
/// If `--formula` is not specified and no input file is provided and if `stdin` is empty, then the
/// program aborts with a non-zero exit code.
///
/// If the formula is not well-formed then the program also aborts with a non-zero exit code.
#[paw::main]
pub fn main(args: Args) -> io::Result<()> {
    logger::setup(args.debug);

    info!(
        "{}: v{}",
        "Propositional Tableau Solver".cyan().bold().underline(),
        VERSION.unwrap_or("unknown version").yellow()
    );

    info!("arguments provided\n {:#?}", &args);

    let mut inputs: Vec<String> = Vec::new();

    // Input precedence:
    //
    // 1. Single formula input from `--formula`, `-c`.
    // 2. File input from `--input`, `-i`.
    // 3. Standard input from `stdin`.
    if let Some(input) = &args.single_formula {
        inputs.push(input.to_string());
    } else if let Some(input_path) = &args.input_file {
        let file = fs::File::open(input_path)?;
        let reader = io::BufReader::new(&file);

        for line in reader.lines() {
            if let Ok(line) = line {
                inputs.push(line);
            } else {
                error!(
                    "I/O error encountered when trying to read from {:#?}",
                    &file
                );
                std::process::exit(5);
            }
        }
    } else {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        for line in stdin.lines() {
            if let Ok(line) = line {
                inputs.push(line);
            } else {
                error!("I/O error encountered when trying to read from STDIN");
                std::process::exit(5);
            }
        }
    }

    debug!("raw inputs:\n{:#?}", &inputs);

    let formulas: Vec<PropositionalFormula> = inputs
        .iter()
        .map(|f| match parser::parse(f) {
            Ok(f) => f,
            Err(_) => {
                error!("ill-formed formula: {:#?}", &f);
                std::process::exit(22);
            }
        })
        .collect();

    debug!("parsed formulas:\n{:#?}", &formulas);

    let results: Vec<bool>;

    let mode = args.mode.map(|c| CliOutputMode::try_from(c).ok()).flatten();

    match mode {
        Some(CliOutputMode::Validity) => {
            info!("using validity mode");
            results = formulas.iter().map(is_valid).collect();
        }
        _ => {
            info!("using satisfiability mode");
            // Default to satisfiability mode.
            results = formulas.iter().map(is_satisfiable).collect();
        }
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for result in results {
        stdout.write_fmt(format_args!("{:?}\n", result))?;
    }

    Ok(())
}
