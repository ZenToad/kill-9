#![allow(dead_code)]
#![allow(unused_imports)]

use clap::*;
use std::path::PathBuf;

//////////////////////////////////////////
// quick start
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliQuickStart {
    // because this doesn't have an #[arg]
    // its a positional arg
    name: Option<String>,

    // sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    // ture debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    // sub command (What even is this?)
    #[command(subcommand)]
    command: Option<QSCommands>,
}

#[derive(Subcommand)]
enum QSCommands {
    // does testing things
    Test {
        // list test values
        #[arg(short, long)]
        list: bool,
    },
}

fn cli_quick_start() {
    let cli = CliQuickStart::parse();

    // check for values for optional flags
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // path is an optional option
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Why are you like this?"),
    }

    // you can check for the existence of subcommands, and if found
    // use their matches just like a top level command
    match &cli.command {
        Some(QSCommands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}

//////////////////////////////////////////
// cli_main()
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    // --one <ONE>
    #[arg(long)]
    one: String,

    // -t, --two <TWO>
    #[arg(short, long)]
    two: String,

    #[arg(short = '3', long = "long_af")]
    three: String,

    // $ --name bob --name=john -n tom -n=chris -nsteve
    // OUT name: ["bob", "john", "tom", "chris", "steve"]
    #[arg(short, long)]
    name: Vec<String>,
}

fn cli_main() {
    let cli = Cli::parse();
    println!("one: {:?}", cli.one);
    println!("two: {:?}", cli.two);
    println!("three: {:?}", cli.three);
    println!("name: {:?}", cli.name);
}

//////////////////////////////////////////
// Positionals
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliPosition {
    // this doesn't have any #[arg] or anything so must be
    // this position
    name: String,
}

fn positionals() {
    let cli = CliPosition::parse();
    println!("name: {:?}", cli.name);
}

//////////////////////////////////////////
// flags()
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliFlags {
    #[arg(short, long)]
    verbose: bool,
}

fn flags() {
    let cli = CliFlags::parse();
    println!("Verbose: {:?}", cli.verbose);
}

//////////////////////////////////////////
// Subcommands
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliSubcommand {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
}

fn subcommands() {
    let cli = CliSubcommand::parse();

    match &cli.command {
        Commands::Add { name } => {
            println!("'add' was used, name is: {name:?}");
        }
    }
}

fn main() {
    // cli_quick_start();
    // cli_main();
    // flags();
    // positionals();
    subcommands();
}
