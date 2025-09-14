#![allow(unused_imports)]

use std::{
    error::Error,
    io::{self, Write},
    thread,
    time::Duration,
};

use console::{Style, Term, style};
use dialoguer::{
    Confirm, Input, Select,
    theme::{ColorfulTheme, SimpleTheme},
};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

struct App {
    term: Term,
}

impl App {
    fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }
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
    let app = App::new();
    app.term.hide_cursor().unwrap();

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
{0}
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
    pb.finish_and_clear();
    app.term.write_line(&result).unwrap();
    //
    wizard(app);
}

fn wizard(app: App) {
    // banner
    app.term.show_cursor().unwrap();

    write_help_line(&app);

    app.term.set_title("DeEz NuTs");
    loop {
        if let Ok(result) = Input::<String>::new()
            .with_prompt("mnt >")
            .allow_empty(true)
            .interact_text()
        {
            match result.as_str() {
                "clear" => {
                    clear_screen(&app);
                }
                "help" => {
                    help(&app);
                }
                "exit" | "quit" => {
                    app.term.write_line("Bye!!!").unwrap();
                    return;
                }
                _ => {
                    println!("Unknown command: {result}");
                    write_help_line(&app);
                }
            }
        }
    }
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

fn help(app: &App) {
    let t = &app.term;
    let _ = t.write_line(&format!("{}", style("commands:").bold()));
    let _ = t.write_line("  help                - show this help");
    let _ = t.write_line("  clear               - clear the screen");
    let _ = t.write_line("  exit | quit         - leave");
}
