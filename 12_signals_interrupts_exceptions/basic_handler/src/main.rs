#![cfg(not(windows))]

use std::process;
use std::time::{Duration};
use std::thread::{sleep};
use libc::{SIGTERM, SIGUSR1, SIGINT};

static mut SHUT_DOWN: bool = false;

fn main() {

    let pid = process::id();
    println!("process_id = {}", pid);

    register_signal_handlers();

    let delay = Duration::from_secs(1);

    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;   
            }
        }

        sleep(delay);

        let signal = SIGUSR1;
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    
    // If SIGTERM was detected, this will invoke function pointer handle_sigterm
    // If SIGUSR1 was detected, this will invoke function pointer handle_sigusr1
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
        libc::signal(SIGINT, handle_sigint as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    register_signal_handlers();
    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}

#[allow(dead_code)]
fn handle_sigint(_signal: i32) {
    register_signal_handlers();
    println!("SIGINT");
    println!("SIGINT");
    println!("SIGINT\n");
    println!("Forceful Shutdown...");
    unsafe {
        SHUT_DOWN = true;
    }
}