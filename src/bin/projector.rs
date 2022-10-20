use rust_polyglot_projector_cli_tool::{opts::Opts, config::Config};

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}
