mod run;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use run::{run, RunArgs};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Runs a given source using the integrated interpreter
    Run(RunArgs),
    /// Compiles the source to either bytecode for the VM or machine targets
    Compile(CompileArgs),
    /// Starts a new REPL instance, good for prototyping with Arkoi
    Repl(ReplArgs),
}

#[derive(Args)]
struct CompileArgs {
    input_file: PathBuf,
    output_file: PathBuf,
}

#[derive(Args)]
struct ReplArgs {}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Run(args) => run(args),
        Mode::Compile(_) => {}
        Mode::Repl(_) => {}
    }
}
