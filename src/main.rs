// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

mod lib;

// SYSTEM INCLUDES

use std::fs::File;
use std::io::{self, BufReader};
use lib::{SOURCE_ENDIAN, ENDIANESS, ENDIAN_READ_ERR};
use std::env as ENV;
use std::process as PROC;

// EXAMPLE DEMONSTRATION OF PARSING A BINARY FILE FOR COMPILED ASSEMBLY CODE

#[repr(C)]
#[derive(Debug)]
struct INSTR 
{
    OPCODE: u16, 
    OPERAND: u16,
}


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
