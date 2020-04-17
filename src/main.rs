use colored::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn main() {
    println!(
        "{}: v{}",
        "Propositional Tableau Solver".cyan().bold().underline(),
        VERSION.unwrap_or("unknown version").yellow()
    );
}
