extern crate diesel;

use clap::Parser;

use anyhow::{anyhow, Result};
use commands::add::AddSubcommand;
use commands::edit::EditSubcommand;
use commands::log::LogSubcommand;
use commands::frames::FramesSubcommand;
use commands::start::StartSubcommand;
use commands::status::StatusSubcommand;
use commands::stop::StopSubcommand;
use commands::projects::ProjectsSubcommand;
use commands::remove::RemoveSubcommand;
use commands::{MyCommand, Output};
use aze::database::{establish_connection, run_migrations};
pub mod commands;
mod config;

#[derive(Debug, Parser)]
#[clap(name = "aze")]
#[clap(
    about = "aze is a tool aimed at helping you monitoring your time.\n\nYou just have to tell aze when you start working on your project with the `start` command, and you can stop the timer when you're done with the `stop` command."
)]
struct Cli {
    #[clap(
        global = true,
        long = "version",
        help = "Show the version and exit.",
        exclusive = true,
        display_order = 9999
    )]
    version_flag: bool,

    #[clap(
        global = true,
        long = "color",
        help = "Color output",
        conflicts_with = "no-color",
        display_order = 9998
    )]
    color: bool,

    #[clap(
        global = true,
        long = "no-color",
        help = "Don't color output",
        conflicts_with = "color",
        display_order = 9997
    )]
    no_color: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Add(AddSubcommand),
    Start(StartSubcommand),
    Stop(StopSubcommand),
    Status(StatusSubcommand),
    Log(LogSubcommand),
    Edit(EditSubcommand),
    Frames(FramesSubcommand),
    Projects(ProjectsSubcommand),
    Remove(RemoveSubcommand)
}

fn main() -> Result<()> {
    let mut conn = establish_connection();
    let migrations = run_migrations(&mut conn);
    if migrations.is_err() {
        return Err(anyhow!("Could not update internal database"));
    }

    let args = Cli::parse();
    if args.version_flag {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let output: Output = Output {
        out: &mut std::io::stdout(),
    };


    match args.command {
        Commands::Add(command) => command.run(output),
        Commands::Start(command) => command.run(output),
        Commands::Stop(command) => command.run(output),
        Commands::Status(command) => command.run(output),
        Commands::Log(command) => command.run(output),
        Commands::Edit(command) => command.run(output),
        Commands::Frames(command) => command.run(output),
        Commands::Projects(command) => command.run(output),
        Commands::Remove(command) => command.run(output),
    }
}
