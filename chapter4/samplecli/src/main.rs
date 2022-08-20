use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your Name",
    about = "Super awesome sample RPN calculater"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long, value_parser)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE", value_parser)]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
