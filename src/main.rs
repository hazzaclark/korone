// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

mod lib;

// SYSTEM INCLUDES

use std::env as ENV;
use std::process as PROC;

fn main()
{
    let args: Vec<String> = ENV::args().collect();

    if args.len() < 2
    {
        eprintln!
        (
            "Usage: {} <INPUT_FILE> <OUTPUT_FMT>\n", args[0]
        );

        PROC::exit(1);
    }

    let INPUT = &args[1];
}