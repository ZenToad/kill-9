use std::net::Ipv4Addr;

use clap::{ArgAction, Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliParser {
    // // #[arg(short, long)]
    // ip_addr: Option<Ipv4Addr>,
    //
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // verbose: u8,
    #[arg(short='s', long, value_enum, action=ArgAction::Append)]
    scan: Mode,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    #[value(name = "V", help = "Version detection: -sV")]
    V,
    #[value(name = "S", help = "Version detection: -sS")]
    S,
    #[value(name = "U", help = "Version detection: -sU")]
    U,
}

fn main() {
    let cli = CliParser::parse();
    // if let Some(ipaddr) = cli.ip_addr {
    //     println!("ip addr: {:?}", ipaddr);
    // }
    // println!("verbose {:?}", cli.verbose);
    match cli.scan {
        Mode::V => println!("V"),
        Mode::U => println!("U"),
        Mode::S => println!("S"),
    }
}
