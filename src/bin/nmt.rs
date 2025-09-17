#![allow(unused_imports)]
use clap::*;
use std::{
    error::Error,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use console::{Style, Term, style};
use dialoguer::{
    Confirm, Input, Select,
    theme::{ColorfulTheme, SimpleTheme},
};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    fast_start: bool,
}

struct App {
    term: Term,
    modules: Vec<Module>,
    using: Option<u32>,
    exploit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            term: Term::stdout(),
            using: None,
            modules: genearate_modules(),
            exploit: false,
        }
    }
}

struct Module {
    name: String,
    date: String,
    rank: ModuleRank,
    check: bool,
    desc: String,
}

#[derive(Debug)]
enum ModuleRank {
    average,
    normal,
    good,
    great,
}

fn genearate_modules() -> Vec<Module> {
    vec![
        Module {
            name: "Cool Name".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
        Module {
            name: "ASDF".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
        Module {
            name: "ASDF".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
        Module {
            name: "ASDF".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
        Module {
            name: "ASDF".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
        Module {
            name: "ASDF".to_string(),
            date: "DATE".to_string(),
            rank: ModuleRank::good,
            check: false,
            desc: "Very cool description".to_string(),
        },
    ]
}

////////////////////////////////////////////////////////////////////////////////////
//  font: Pagga
//
// ░█▀█░█▀▀░█▀█░█▀█░░░█▄█░▀█▀░█▀█░█▀█░▀█▀░█▀█░█░█░█▀▄
// ░█░█░█▀▀░█░█░█░█░░░█░█░░█░░█░█░█░█░░█░░█▀█░█░█░█▀▄
// ░▀░▀░▀▀▀░▀▀▀░▀░▀░░░▀░▀░▀▀▀░▀░▀░▀▀▀░░▀░░▀░▀░▀▀▀░▀░▀
//
//
// Neon Minotaur - NMT
//
// km9 version of metasploit
fn main() {
    let cli = Cli::parse();

    let mut app = App::new();
    app.term.hide_cursor().unwrap();

    if !cli.fast_start {
        do_long_startup();
    }

    let finish_message = r#"░█▀█░█▀▀░█▀█░█▀█░░░█▄█░▀█▀░█▀█░█▀█░▀█▀░█▀█░█░█░█▀▄
░█░█░█▀▀░█░█░█░█░░░█░█░░█░░█░█░█░█░░█░░█▀█░█░█░█▀▄
░▀░▀░▀▀▀░▀▀▀░▀░▀░░░▀░▀░▀▀▀░▀░▀░▀▀▀░░▀░░▀░▀░▀▀▀░▀░▀

Version x.y.z
Ready...
> access security
access: PERMISSION DENIED.
> access security grid.
access: PERMISSION DENIED.
> access main security grid.
access: PERMISSION DENIED....and...
{0}
{0}
{0}

       =[ neon minotaur v6.9.3-dev                    ]
+ -- --=[ 2222 exploits - 1177 auxilary - 389 post    ]
+ -- --=[ 867 payloads - 45 encoders - 11 nops        ]
+ -- --=[ 9 evasion                                   ]

Neon Minotaur tip: After running XXX, be sure to
lick deeznuts:)
"#;
    let result = finish_message.replace(
        "{0}",
        &style("YOU DIDN'T SAY THE MAGIC WORD!").red().to_string(),
    );
    // pb.finish_and_clear();
    app.term.write_line(&result).unwrap();
    wizard(app);
}

fn tokenize_simple(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn do_long_startup() {
    // ok, how do we do that fancy ass loading screen?
    // don't have to match exactly
    // do this for a couple seconds
    // [*] Starting Neon Minotaur... [SPINNER]
    let pb = ProgressBar::new_spinner();
    pb.set_draw_target(ProgressDrawTarget::stderr());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{msg} {spinner:.blue}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "     ",
            ]),
    );
    pb.set_message("[*] Starting Neon Minotaur...");
    thread::sleep(Duration::from_secs(2));

    // Then it spits out a bunch of files and warnings and junk

    // Now it does this again after all the spew
    // do this for a couple seconds
    // [*] Starting Neon Minotaur... [SPINNER]

    // output once finished:
}

fn reverse_shell() {
    //
    println!("We're In!!!");
    loop {
        if let Ok(line) = Input::<String>::new()
            .with_prompt("we're in!>")
            .allow_empty(true)
            .interact_text()
        {
            match line.to_lowercase().trim() {
                "quit" | "exit" => {
                    return;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

fn wizard(mut app: App) {
    // banner
    app.term.show_cursor().unwrap();

    write_help_line(&app);

    loop {
        if app.exploit {
            reverse_shell();
        }
        if let Ok(line) = Input::<String>::new()
            .with_prompt(get_prompt(&app))
            .allow_empty(true)
            .interact_text()
        {
            match line.to_lowercase().trim() {
                "" => {
                    continue;
                }
                "help" => {
                    help(&app);
                }
                "clear" => {
                    clear_screen(&app);
                }
                "quit" | "exit" => {
                    return;
                }
                "search" => {
                    search(&app, &line);
                }
                "use 1" => {
                    app.using = Some(1);
                    println!(
                        "{} Using configured payload windows/meterpreter/reverse_tcp",
                        style("[+]").green()
                    );
                }
                "use 3" => {
                    app.using = Some(3);
                }
                "options" => {
                    show_options(&app);
                }
                "set" => {
                    set_option(&app, &line);
                }
                "run" => {
                    run(&app);
                }
                "exploit" => {
                    exploit(&mut app);
                }
                _ => {
                    println!("Unknown Command: {line}");
                }
            }
        }
    }
}

fn show_options(app: &App) {
    println!("Module options(scanner/smb/smbb_ms17_010):");
    println!();
    println!("Name          Current Setting          Required  Description");
    println!("----          ---------------          --------  -----------");
    println!(
        "CHECK_ARCH    true                     no        Check for architecture on vulnerable hosts"
    );
    println!(
        "CHECK_DOPU    true                     no        Check for DOUBLEPULSAR on vulnerable hosts"
    );
    println!(
        "CHECK_PIPE    false                    no        Check for named pipes on vulnerable hosts"
    );
    println!("NAMES_PIPES   /usr/share/neonminotaur  yes       List of named pipes to check");
    println!("              lists/named_pipes.txt");
    println!("RHOSTS                                 yes       The target host(s)");
    println!("RPORT         445                      yes       The SMB service port (TCP)");
    println!(
        "SMBDomain     .                        no        The Windows domain to use for authentication"
    );
}

fn set_option(app: &App, line: &String) {
    //
    println!("rhosts => 10.2.0.9")
}

fn run(app: &App) {
    if let Some(num) = app.using {
        if num == 3 {
            //
            println!();
            println!(
                "{} 10.2.0.9:455       - Host is likely VULNERABLE to MS17-010! - Windows 5.1 x86 (32-bit)",
                style("[+]").green()
            );
            println!(
                "{} 10.2.0.9:455       - Scanned 1 of 1 hosts (100% complete)",
                style("[+]").blue()
            );
            println!(
                "{} Auxilary module execution completed",
                style("[+]").blue()
            );
        } else if num == 1 {
            println!(
                "{} Using configured payload windows/meterpreter/reverse_tcp",
                style("[+]").blue()
            );
        }
    }
}

fn get_prompt(app: &App) -> String {
    if let Some(num) = app.using {
        if num == 1 {
            format!(
                "nmt auxilary({}) >",
                style("exploit/smb/smbb_ms17_010").red()
            )
        } else {
            format!(
                "nmt auxilary({}) >",
                style("scanner/smb/smbb_ms17_010").red()
            )
        }
    } else {
        "nmt >".to_string()
    }
}

fn search(app: &App, line: &String) {
    //
    println!("Matching Modules");
    println!("================");
    println!();
    println!("#  Name           Disclosure Date    Rank    Check    Description");
    println!("-  ----           ---------------    ----    -----    -----------");

    for (index, module) in app.modules.iter().enumerate() {
        println!(
            "{} {} {} {:?} {} {}",
            index, module.name, module.date, module.rank, module.check, module.desc
        );
    }
    println!(
        "Interact with a module by name or index. For example {}, {} or {}",
        style("info 7").green(),
        style("use 7").green(),
        style("use exploit/windows/smb/smb_doublepular_rce").green(),
    );
}

fn exploit(app: &mut App) {
    //
    println!(
        "{} Started reverse TCP handler on 10.0.2.4:4444",
        style("[*]").blue()
    );
    println!(
        "{} 10.0.2.4:445 - Target OS: Windows 5.1",
        style("[*]").blue()
    );
    println!(
        "{} 10.0.2.4:445 - Filling barrel with fish... done",
        style("[*]").blue()
    );
    println!(
        "{} 10.0.2.4:445 - Service started su;ccessfully...",
        style("[+]").green()
    );
    println!(
        "{} Sending stage (175686 bytes) to 10.0.2.9",
        style("[*]").blue()
    );

    app.exploit = true;
}

fn help(app: &App) {
    let t = &app.term;
    let _ = t.write_line(&format!("{}", style("commands:").bold()));
    let _ = t.write_line("  clear               - clear the screen");
    let _ = t.write_line("  exit | quit         - leave");
    let _ = t.write_line("  help                - show this help");
    let _ = t.write_line("  search              - search for ...");
    let _ = t.write_line("  use 1               - use exploits");
    let _ = t.write_line("  use 3               - use auxilary");
    let _ = t.write_line("  options             - show options");
    let _ = t.write_line("  set                 - set option");
    let _ = t.write_line("  run                 - run selection");
    let _ = t.write_line("  exploit             - run exploit");
}

fn write_help_line(app: &App) {
    let _ = app.term.write_line(&format!(
        "interactive shell — type {} or {}",
        style("help").yellow(),
        style("exit").yellow()
    ));
}

fn clear_screen(app: &App) {
    app.term.clear_screen().unwrap();
}
