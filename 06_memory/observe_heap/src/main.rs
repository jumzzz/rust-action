use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::*;
use rand::prelude::*;

use std::alloc::{GlobalAlloc, System, Layout};

use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();

        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}",
            bytes_requested,
            time_taken.as_nanos()
        );
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("Deallocated {:?} bytes at {:p}", layout.size(), ptr);
        System.dealloc(ptr, layout)
    }
}

struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    he
}

fn main() {
    println!("Hello, world!");
}
