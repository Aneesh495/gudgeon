//! Gudgeon Command Line Interface
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Gudgeon Autonomous Surface Vessel Navigation & Control Engine");
        println!("Usage: gudgeon <command> [args...]");
        println!("Commands:");
        println!("  run <file.lol>   Execute a LOLCODE script");
        println!("  verify           Run qualification test gates");
        println!("  test             Run test suite");
        println!("  sim              Run 3-DOF vessel simulation");
        println!("  bench            Run execution benchmarks");
        process::exit(0);
    }
}
