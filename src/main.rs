use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;

mod cli;
mod metadata;
mod version;

fn main() -> Result<()> {
    // get cargo metadata
    let metadata = MetadataCommand::new().exec()?.into();

    // parse args
    let args = cli::Cli::parse();

    args.execute(&metadata)?;

    Ok(())
}
