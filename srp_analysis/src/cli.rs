use structopt::StructOpt;
use std::process::Command;

use crate::analysis::*;
use crate::common::Tasks;
use crate::templating::write_report_to_file;

#[derive(StructOpt)]
struct CliOpts {
    #[structopt(subcommand)]
    cmd: CliCommand,
}

#[derive(StructOpt)]
enum CliCommand {
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
        CliCommand::Generate(g) => generate(g, tasks),
    }
}

fn generate(g: Generate, tasks: &Tasks) -> Result<(), String> {
    let res = run_analysis(tasks, g.approximation)?;
    let filename = g.name + ".html";

    if g.verbose {
        println!("{:?}", res);
    }

    write_report_to_file(&res, &filename)?;
    open_report(&filename);
    Ok(())
}

fn open_report(filename: &String) {
    Command::new("firefox")
        .arg(filename)
        .spawn()
        .expect("Could not open file or you don't have Firefox installed!");
}
