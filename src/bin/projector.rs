use rust_polyglot_projector_cli_tool::opts::Opts;
use clap::Parser;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
