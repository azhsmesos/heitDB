use std::process::exit;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("HeitDB")
        .version("0.1")
        .author("azh")
        .about("A KV store DB")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            println!("unimplemented");
            exit(1);
        },
        ("get", Some(_matches)) => {
            println!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
