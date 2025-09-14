// Directory Killer!!!
use clap::Parser;
use std::path::PathBuf;

/// DIRK IS a Directory Killer!!!
///
/// It looks for existing (and/or hidden) Web Objects.
/// It basically works by launching a dictionary-based attack
/// against a web server and analyzing the response.
#[derive(Parser, Debug)]
#[command(
    name = "dirk",
    version,
    about = "Directory Killer",
    long_about = None,
    // Show a manpage-ish layout:
    help_template = "\
NAME
    {name} - {about}

SYNOPSIS
    {usage}

DESCRIPTION
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Cras congue nulla et eros congue commodo. Aliquam venenatis, 
ligula vitae lobortis porttitor, ipsum metus egestas risus, 
id rutrum leo risus at justo. Curabitur posuere sodales elit et lacinia. 
Lorem ipsum dolor sit amet.

OPTIONS
{all-args}
",
    // Replace Clap's auto-usage with your preferred synopsis line:
    // override_usage = "dirb <url_base> <url_base> [<wordlist_file(s)>] [options]"
)]
struct Cli {
    /// Exactly two base URLs
    // #[arg(value_name = "url_base", num_args = 2, required = true)]
    // url_bases: Vec<String>,

    /// Optional wordlist files
    #[arg(value_name = "wordlist_file", num_args = 0..)]
    wordlists: Vec<PathBuf>,

    /// Custom USER_AGENT
    #[arg(short = 'a', value_name = "agent_string")]
    agent: Option<String>,

    /// Don't squash or merge sequences of /../ or /./
    #[arg(short = 'b')]
    no_squash: bool,
}

fn run() {
    let ui = r#"
DIRK v4.20
By The Zen Toad

START_TIME: Mon Dec 25 11:11:12 2025
URL_BASE: http://192.168.0.123/
WORKDLIST_FILES: C:\usr\tmp\wordsxxx.txt

GENERATED WORDS: 69

── Scanning URL http://192.168.0.123/ ──
+ http://192.168.0.123/cgi-bin/ (CODE:403|SIZE:288)
⇒ DIRECTORY: http://192.168.0.123/chat/
⇒ DIRECTORY: http://192.168.0.123/drupal/
⇒ DIRECTORY: http://192.168.0.123/phpmyadmin/
+ http://192.168.0.123/server-status/ (CODE:403|SIZE:293)
⇒ DIRECTORY: http://192.168.0.123/uploads/

── Entering directory: http://192.168.0.123/chat ──
+ http://192.168.0.123/chat/index.php (CODE:200|SIZE:771)
-→ Testing: http://192.168.0.123/chat/[This changes while scanning]

────────────────────────
END_TIME: Mon Dec 25 11:11:12 2025
"#;

    println!("{ui}");
}

fn main() {
    let _cli = Cli::parse();
    run();
}

/*
*
* nmt.exe Neon Minotaur
*/
