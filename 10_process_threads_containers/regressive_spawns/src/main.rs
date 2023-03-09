use clap::{Arg, App};
use std::{string::ParseError};
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
struct Params {
    num_threads: u64,
    sample_per_step: u64,
    delay: u64, 
}

impl Params {
    fn build(
        num_thread_arg: &String,
        sample_per_step_arg: &String,
        delay_arg: &String, 
    ) -> Result<Params, ParseError> {
        Ok(
            Params { 
                num_threads: num_thread_arg.parse::<u64>().unwrap(), 
                sample_per_step: sample_per_step_arg.parse::<u64>().unwrap(), 
                delay: delay_arg.parse::<u64>().unwrap(), 
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
            Arg::with_name("delay")
            .short("d")
            .default_value("20")
            .help("Delay in milliseconds")
        ).arg(
            Arg::with_name("sample-per-step")
            .short("s")
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

    let delay_arg = app.value_of("delay")
                            .unwrap()
                            .to_string();
    
    Params::build(
        &num_thread_arg, 
        &sample_per_step_arg, 
        &delay_arg
    ).expect("Parameters must be an unsigned integer.")
    
}

fn spawn_threads(params: Params) {
    println!("number_of_threads,sample_number,delay");
    // Generate thread count
    for thread_count in 1..params.num_threads + 1 {
        // Generate n number of sample per thread count
        for n in 0..params.sample_per_step {
            let mut handles = Vec::with_capacity(thread_count as usize);
            let start = time::Instant::now();

            // Spawn thread based on thread count
            for _ in 0..thread_count {
                let handle = thread::spawn(move || {
                    let pause = time::Duration::from_millis(
                        params.delay
                    );
                    thread::sleep(pause);
                });
                handles.push(handle);
            }

            // Wait for all threads to get finished
            // if handles have ampersand &handles
            // a move will occur
            for handle in handles {
                handle.join().unwrap();
            }

            let finish = time::Instant::now();
            let time_diff = finish.duration_since(start).as_micros() as f64;
            println!("{},{},{:02}", thread_count, n, time_diff / 1000.0);
        }
    }
}


fn main() {
    let params = parse_args();
    spawn_threads(params); 
}
