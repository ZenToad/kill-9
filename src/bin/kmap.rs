use console::style;
use std::net::Ipv4Addr;

use clap::{ArgAction, Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct CliParser {
    // #[arg(short, long)]
    ip_addr: Option<Ipv4Addr>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short='s', long, value_enum, action=ArgAction::Append)]
    scan: Option<Mode>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    #[value(name = "V", help = "Version detection: -sV")]
    Version,
    #[value(name = "S", help = "Version detection: -sS")]
    S,
    #[value(name = "U", help = "Version detection: -sU")]
    U,
}

fn run_version_scan(ip: Option<Ipv4Addr>) {
    // what goes here?
    println!("Starting Kmap 4.2 (https://assballs)");
    if let Some(ipaddr) = ip {
        println!("Kmap scan report for {:?}", ipaddr);
    } else {
        println!("Kmap scan report for {:?}", "127.0.0.1");
    }
    println!("Host is up (0.000042s latency).");
    println!("Ports scanned: 0 - 65535");
    println!(
        "{:<10} {:<8} {:<12} {}",
        "PORT", "STATE", "SERVICE", "VERSION"
    );
    println!(
        "{:<10} {:<8} {:<12} {}",
        "21/tcp",
        style("open").green(),
        "ftp",
        "ProFTPD 1.3.5"
    );
    println!(
        "{:<10} {:<8} {:<12} {}",
        "3000/tcp",
        style("closed").red(),
        "netbois-ssn",
        "ProFTPD 1.3.5",
    );
    println!("Device type: {}", "general purpose");
    println!("Running: {}", "Linux 3.X|4.X");
}

fn main() {
    let cli = CliParser::parse();
    // if let Some(ipaddr) = cli.ip_addr {
    //     println!("ip addr: {:?}", ipaddr);
    // }
    // println!("verbose {:?}", cli.verbose);
    //
    if let Some(clii) = cli.scan {
        match clii {
            Mode::Version => {
                run_version_scan(cli.ip_addr);
            }
            Mode::U => {
                println!("U");
            }
            Mode::S => {
                println!("S");
            }
        }
    }
}
