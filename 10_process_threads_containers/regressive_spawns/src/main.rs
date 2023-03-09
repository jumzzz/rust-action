use clap::{Arg, App};
use std::{string::ParseError};


#[derive(Debug, Copy, Clone)]
struct Params {
    num_threads: u32,
    sample_per_step: u32,
    num_step: u32,
    delay: u32, 
}

impl Params {
    fn build(
        num_thread_arg: &String,
        sample_per_step_arg: &String,
        num_step_arg: &String,
        delay_arg: &String, 
    ) -> Result<Params, ParseError> {
        Ok(
            Params { 
                num_threads: num_thread_arg.parse::<u32>().unwrap(), 
                sample_per_step: sample_per_step_arg.parse::<u32>().unwrap(), 
                num_step: num_step_arg.parse::<u32>().unwrap(), 
                delay: delay_arg.parse::<u32>().unwrap(), 
            }
        )
    }
}

fn parse_args() -> Params {
    let app = App::new("regressive_spawns")
        .about("Observe Thread Spawn Regression")
        .arg(
            Arg::with_name("num-threads")
            .short("n")
            .default_value("100")
            .help("Number of maximum threads")
        )
        .arg(
            Arg::with_name("num-steps")
            .short("s")
            .default_value("1")
            .help("Number of Increasing Steps per iteration")
        )
        .arg(
            Arg::with_name("delay")
            .short("d")
            .default_value("20")
            .help("Delay in milliseconds")
        ).arg(
            Arg::with_name("sample-per-step")
            .default_value("5")
            .help("Number of samples per step")
        )
        .get_matches();

    let num_thread_arg = app.value_of("num-threads")
                            .unwrap()
                            .to_string();

    let sample_per_step_arg = app.value_of("sample-per-step")
                                .unwrap()
                                .to_string();

    let num_step_arg = app.value_of("num-steps")
                            .unwrap()
                            .to_string();

    let delay_arg = app.value_of("delay")
                            .unwrap()
                            .to_string();
    
    Params::build(
        &num_thread_arg, 
        &sample_per_step_arg, 
        &num_step_arg, 
        &delay_arg
    ).expect("Parameters must be an unsigned integer.")
    
}




fn main() {

    let args = parse_args();
    println!("{:?}", args);
    

}
