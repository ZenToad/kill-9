#![allow(dead_code)]
use console::{Style, Term};
use dialoguer::{
    BasicHistory, Completion, Confirm, Editor, FuzzySelect, History, Input, MultiSelect, Password,
    Select, Sort, theme::ColorfulTheme,
};
use std::{collections::VecDeque, error::Error, net::IpAddr, process};

fn main() {
    // buffered();
    // completion();
    // confirm();
    // editor();
    // fuzzy();
    // history();
    // history_custom();
    // input();
    // paging();
    // password();
    // select();
    wizard();
}

#[derive(Debug)]
struct Config {
    interface: IpAddr,
    hostname: String,
    use_acme: bool,
    private_key: Option<String>,
    cert: Option<String>,
}

fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(None);
    }

    let interface = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    let hostname = Input::with_theme(&theme)
        .with_prompt("Hostname")
        .interact()?;

    let tls = Select::with_theme(&theme)
        .with_prompt("Configure TLS")
        .default(0)
        .item("automatic with ACME")
        .item("manual")
        .item("no")
        .interact()?;

    let (private_key, cert, use_acme) = match tls {
        0 => (Some("acme.pkey".into()), Some("acme.cert".into()), true),
        1 => (
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to private key")
                    .interact()?,
            ),
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to certificate")
                    .interact()?,
            ),
            false,
        ),
        _ => (None, None, false),
    };

    Ok(Some(Config {
        hostname,
        interface,
        private_key,
        cert,
        use_acme,
    }))
}

fn wizard() {
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => println!("{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
}

fn select() {
    let list = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let sorted = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[sorted[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[sorted[sorted.len() - 1]]);

    let sorted = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .max_length(2)
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[sorted[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[sorted[sorted.len() - 1]]);
}

fn password() {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.chars().count() > 3 {
                Ok(())
            } else {
                Err("Password must be longer than 3")
            }
        })
        .interact()
        .unwrap();

    println!(
        "Your password is {} characters long",
        password.chars().count()
    );
}

// if the terminal height isn't enough for all these options
// it will split them into pages
fn paging() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Carrots",
        "Peas",
        "Pistacio",
        "Mustard",
        "Cream",
        "Banana",
        "Chocolate",
        "Flakes",
        "Corn",
        "Cake",
        "Tarte",
        "Cheddar",
        "Vanilla",
        "Hazelnut",
        "Flour",
        "Sugar",
        "Salt",
        "Potato",
        "French Fries",
        "Pizza",
        "Mousse au chocolat",
        "Brown sugar",
        "Blueberry",
        "Burger",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}

fn input() {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()
        .unwrap();

    println!("Hello {}!", input);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || (force.as_ref() == Some(input)) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Email: {}", mail);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your planet")
        .default("Earth".to_string())
        .interact_text()
        .unwrap();

    println!("Planet: {}", mail);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your galaxy")
        .with_initial_text("Milky Way".to_string())
        .interact_text()
        .unwrap();

    println!("Galaxy: {}", mail);
}

fn history_custom() {
    println!("Use 'exit' to quit the prompt");
    println!("In this example, history is limited to 4 entries");
    println!("Use the Up/Down arrows to scroll through history");
    println!();

    let mut history = MyHistory::default();

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("dialoguer")
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                process::exit(0);
            }
            println!("Entered {}", cmd);
        }
    }
}

struct MyHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for MyHistory {
    fn default() -> Self {
        MyHistory {
            max: 4,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}

fn history() {
    println!("Use 'exit' to quit the prompt");
    println!("In this example, history is limited to 8 entries and contains no duplicates");
    println!("Use the Up/Down arrows to scroll through history");
    println!();

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("dialoguer")
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                process::exit(0);
            }
            println!("Entered {}", cmd);
        }
    }
}

fn fuzzy() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Carrots",
        "Peas",
        "Pistacio",
        "Mustard",
        "Cream",
        "Banana",
        "Chocolate",
        "Flakes",
        "Corn",
        "Cake",
        "Tarte",
        "Cheddar",
        "Vanilla",
        "Hazelnut",
        "Flour",
        "Sugar",
        "Salt",
        "Potato",
        "French Fries",
        "Pizza",
        "Mousse au chocolat",
        "Brown sugar",
        "Blueberry",
        "Burger",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}

fn editor() {
    if let Some(rv) = Editor::new()
        .executable("nvim")
        .edit("Enter a commit message")
        .unwrap()
    {
        println!("Your message:");
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
}

fn confirm() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really want to continue?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really want to continue?")
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really want to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really really want to continue?")
        .interact_opt()
        .unwrap()
    {
        Some(true) => println!("Looks like you want to continue"),
        Some(false) => println!("nevermind then :("),
        None => println!("Ok, we can start over later"),
    }

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really really really want to continue?")
        .default(true)
        .wait_for_newline(true)
        .interact_opt()
        .unwrap()
    {
        Some(true) => println!("Looks like you want to continue"),
        Some(false) => println!("nevermind then :("),
        None => println!("Ok, we can start over later"),
    }
}

struct MyCompletion {
    options: Vec<String>,
}

impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: vec![
                "orange".to_string(),
                "apple".to_string(),
                "banana".to_string(),
            ],
        }
    }
}

impl Completion for MyCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();
        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

fn completion() {
    //
    println!("Use the Right arrow or Tab to complete your command");

    let completion = MyCompletion::default();

    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("my prompt")
        .completion_with(&completion)
        .interact_text()
        .unwrap();
}

fn buffered() {
    let items = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    println!("All the following controls are run in a buffered terminal");
    Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact_on(&term)
        .unwrap();

    let _: String = Input::with_theme(&theme)
        .with_prompt("Your name")
        .interact_on(&term)
        .unwrap();

    Select::with_theme(&theme)
        .with_prompt("Pick an item")
        .items(items)
        .interact_on(&term)
        .unwrap();

    MultiSelect::with_theme(&theme)
        .with_prompt("Pick some items")
        .items(items)
        .interact_on(&term)
        .unwrap();

    Sort::with_theme(&theme)
        .with_prompt("Order these items")
        .items(items)
        .interact_on(&term)
        .unwrap();
}
