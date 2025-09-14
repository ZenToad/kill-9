#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::Result;
use console::{Key, Term, style};
use std::io;
use std::{io::Write, thread, time::Duration};

fn main() -> Result<()> {
    // colors();
    // colors256();
    // cursor_at();
    // Ok(())
    // keyboard()
    // term()
    term_vs_println()
}

fn term_vs_println() -> Result<()> {
    let term = Term::stdout();

    // --- Using println! ---
    println!("Checking modules...");
    thread::sleep(Duration::from_secs(1));
    println!("Done.");
    println!("msf6 > "); // <- leaves the extra "Checking..." line above forever

    // --- Using Term ---
    term.write_line("Checking modules...")?;
    thread::sleep(Duration::from_secs(1));
    term.clear_last_lines(1)?; // remove the line
    term.write_line("Done.")?;
    term.write_str("msf6 > ")?; // prompt without newline

    Ok(())
}
fn term() -> Result<()> {
    let term = Term::stdout();
    term.set_title("Counting...");
    term.write_line("Going to do some counting now")?;
    term.hide_cursor()?;
    for x in 0..10 {
        if x != 0 {
            term.move_cursor_up(1)?;
        }
        term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))?;
        thread::sleep(Duration::from_millis(400));
    }
    term.show_cursor()?;
    term.clear_last_lines(1)?;
    term.write_line("Done counting!")?;
    writeln!(&term, "Hello World!")?;

    write!(&term, "To edit: ")?;
    let res = term.read_line_initial_text("default")?;
    writeln!(&term, "\n{res}")?;

    Ok(())
}

fn keyboard() -> Result<()> {
    let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    let term = Term::stdout();
    term.write_line("Press any key. Esc to exit")?;
    loop {
        let key = if raw {
            term.read_key_raw()
        } else {
            term.read_key()
        }?;
        term.write_line(&format!("You pressed {key:?}"))?;
        if key == Key::Escape {
            break;
        }
    }
    Ok(())
}

fn write_chars() -> Result<()> {
    let term = Term::stdout();
    let (height, width) = term.size();
    for x in 0..width {
        for y in 0..height {
            term.move_cursor_to(x as usize, y as usize)?;
            let text = if (x + y) % 2 == 0 {
                format!("{}", style(x % 10).black().on_red())
            } else {
                format!("{}", style(x % 10).red().on_black())
            };

            term.write_str(&text)?;
            thread::sleep(Duration::from_micros(600));
        }
    }
    Ok(())
}

// freakin' cool
fn cursor_at() {
    write_chars().unwrap();
}

fn colors256() {
    for i in 0..=255 {
        print!("{:03} ", style(i).color256(i));
        if i % 16 == 15 {
            println!();
        }
    }

    for i in 0..=255 {
        print!("{:03} ", style(i).black().on_color256(i));
        if i % 16 == 15 {
            println!();
        }
    }
}

fn colors() {
    println!(
        "This is red on black: {:010x}",
        style(42).red().on_black().bold()
    );
    println!("This is reversed: [{}]", style("whatever").reverse());
    println!("This is cyan: {}", style("whatever").cyan());
    eprintln!(
        "This is black bright: {}",
        style("whatever").for_stderr().bright().black()
    );
}
