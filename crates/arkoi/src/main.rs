use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
