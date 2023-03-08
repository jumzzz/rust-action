#[cfg(not(windows))]
use libc;

use chrono::{DateTime, Duration as ChronoDuration, TimeZone, Timelike};
use chrono::{Local, Utc};

use clap::{App, Arg, ArgMatches};

// Standard Libraries
use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

// Useful conversion constants
const NTP_MESSAGE_LENGTH: usize = 48;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

// Note: indicates the data pointed to by the reference
// live as long as the running program. But it can still be coerced
// to a shorter lifetime - Rust by Practice
// 12300 is the default port for NTP
const LOCAL_ADDRESS: &'static str = "0.0.0.0:12300"; 


#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t4 - self.t3);
        duration.num_milliseconds() / 2
    }

    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}


// Try compiling it as -> DateTime<Utc> instead of Self
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);
        
        Utc.timestamp(secs, nanos as u32)
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;

        NTPTimestamp { 
            seconds: secs as u32, 
            fraction: fraction as u32 }
    }
}


struct Clock;
impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    // Understand why <Tz: TimeZone> needs to be explict
    // Why not use DateTime<TimeZone>
    // Without <Tz: TimeZone> you need to add DateTime<dyn TimeZone>
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe {  zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
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
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);

        let maybe_error = std::io::Error::last_os_error();
        let os_error_code = &maybe_error.raw_os_error();

        match os_error_code {
            Some(0) => (),
            Some(_) => eprintln!("Unable to set the time: {:?}", maybe_error),
            None => (),
        }
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}\n", now.timestamp()),
        "rfc2822" => println!("{}\n", now.to_rfc2822()),
        "rfc3339" => println!("{}\n", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
