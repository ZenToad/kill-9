use clap::*;

fn main() {
    // type_validator();
    first_name();
}

fn first_name() {
    let first_name_arg = Arg::new("first-name")
        .long("first-name")
        .help("Please enter your first name with a capital letter.")
        .value_parser(first_name_validator);
    let cmd = Command::new("base").arg(first_name_arg);
    let result = cmd.get_matches();
    println!(
        "Your first name is {0}",
        result.get_one::<String>("first-name").unwrap()
    );
}

fn first_name_validator(value: &str) -> Result<String, std::io::Error> {
    let first_name_regex = regex::Regex::new("[A-Z]\\w+").unwrap();
    if first_name_regex.is_match(value) {
        return Ok(value.to_string());
    }
    Err(std::io::Error::other("message"))
}

fn _type_validator() {
    let coat_types = ["winter", "rain", "hoodie"];

    let coat_arg = Arg::new("coat-type")
        .long("coat-type")
        .value_parser(coat_types);

    let cmd = Command::new("base").arg(coat_arg);
    let result = cmd.get_matches();
    println!(
        "The coat type you requested was {0}",
        result.get_one::<String>("coat-type").unwrap()
    );
}

fn _age_validator() {
    let age_arg = Arg::new("age")
        .long("age")
        .short('a')
        .help("This is the age that will be validated against.")
        .value_parser(value_parser!(u8).range(25..=40));

    let cmd = Command::new("base").arg(age_arg);

    let result = cmd.get_matches();
    println!(
        "The age you passed in was {0}",
        result.get_one::<u8>("age").unwrap()
    );
}
