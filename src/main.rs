// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

// ADD THESE LINES BECAUSE THE COMPILER WAS SCREAMING AT ME
// HEAVEN FORBID YOU HAVE A CODING STYLE ¯\_(ツ)_/¯

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod endian;

// SYSTEM INCLUDES

use std::fs::File;
use std::io::{Read, BufReader};
use crate::endian::{SOURCE_ENDIAN, ENDIAN_READ_ERR};
use std::env as ENV;
use std::process as PROC;

// EXAMPLE DEMONSTRATION OF PARSING A BINARY FILE FOR COMPILED ASSEMBLY CODE

#[repr(C)]
#[derive(Debug)]
pub struct INSTR 
{
    OPCODE: u16, 
    OPERAND: u16,
}

// FUNCTION TO PARSE BINARY FILE

pub fn PARSE_BINARY<R: Read>(READER: &mut R) -> Result<Vec<INSTR>, ENDIAN_READ_ERR> 
{
    let mut INSTRUCTIONS = Vec::new();
    
    loop 
    {
        let OPCODE = match SOURCE_ENDIAN::READ::<R, u16>(READER) 
        {
            Ok(OP) => OP,
            Err(_) => break,
        };
        
        let OPERAND = match SOURCE_ENDIAN::READ::<R, u16>(READER) 
        {
            Ok(OPER) => OPER,
            Err(_) => break,
        };
        
        INSTRUCTIONS.push(INSTR { OPCODE, OPERAND });
    }
    
    Ok(INSTRUCTIONS)
}

// FUNCTION TO PROCESS A BINARY FILE

fn PROC_FILE(F_PATH: &str) -> Result<(), Box<dyn std::error::Error>> 
{
    let FILE = File::open(F_PATH)?;
    let mut READER = BufReader::new(FILE);

    let INSTRUCTIONS = PARSE_BINARY(&mut READER)?;

    for (i, INSTR) in INSTRUCTIONS.iter().enumerate() 
    {
        println!("Instruction {}: {:?}", i, INSTR);
    }

    Ok(())
}

fn main()
{
    let args: Vec<String> = ENV::args().collect();

    if args.len() < 2
    {
        eprintln!
        (
            "Usage: {} <INPUT_FILE>\n", args[0]
        );

        PROC::exit(1);
    }

    let INPUT = &args[1];

    if let Err(err) = PROC_FILE(INPUT)
    {
        eprintln!("Error processing Input File {}", err);
        PROC::exit(1);
    }
}
