use chrono::{DateTime};
use chrono::{Local};

use clap::{App, Arg, ArgMatches};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

// Understand Lifetime and Static Lifetimes
fn parse_args() -> ArgMatches<'static> {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("standard")
                .takes_value(true)
                .possible_values(&[
                    "rfc2822",
                    "rfc3339",
                    "timestamp",
                ])
                .default_value("rfc3339"),

        )
        .arg(
            Arg::with_name("datetime").help(
                "When <action> is 'set', apply <datetime>. \
                Otherwise, ignore.",
            )
        );
    
    app.get_matches()
}

fn main() {
    let args = parse_args();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    println!("Current Args\n");
    println!("action = {}", action);
    println!("std = {}\n", std);

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}\n", now.timestamp()),
        "rfc2822" => println!("{}\n", now.to_rfc2822()),
        "rfc3339" => println!("{}\n", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
