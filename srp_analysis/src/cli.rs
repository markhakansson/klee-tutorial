use structopt::StructOpt;

use crate::analysis::*;
use crate::common::Tasks;
use crate::templating::write_report_to_file;

#[derive(StructOpt)]
struct CliOpts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Generate an analysis report
    Generate(Generate),
}

#[derive(StructOpt)]
struct Generate {
    #[structopt(short, long, default_value = "analysis")]
    /// The filename to export the report as (without file extension)
    name: String,
    #[structopt(short, long)]
    /// Use approximation when running the analysis
    approximation: bool,
    /// Enable verbose output
    #[structopt(short, long)]
    verbose: bool,
}

pub fn cli(tasks: &Tasks) -> Result<(), String> {
    let args = CliOpts::from_args();

    match args.cmd {
        Command::Generate(g) => generate(g, tasks),
    }
}

fn generate(g: Generate, tasks: &Tasks) -> Result<(), String> {
    let res = run_analysis(tasks, g.approximation)?;

    if g.verbose {
        println!("{:?}", res);
    }

    write_report_to_file(&res, g.name)
}
