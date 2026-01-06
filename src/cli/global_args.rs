use clap::Args;

#[derive(Debug, Args)]
pub struct GlobalArgs {
    #[arg(long, help = "Do not write changes to disk")]
    pub dry_run: bool,

    #[arg(long, help = "Apply to the whole workspace")]
    pub workspace: bool,
    #[arg(short, long, value_name = "NAME", help = "Package to modify")]
    pub package: Vec<String>,
}
