// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod endian;

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

pub fn PARSE_BINARY<R: Read>(READER: &mut R, IS_LITTLE_ENDIAN: bool) -> Result<Vec<INSTR>, ENDIAN_READ_ERR> 
{
    let mut INSTRUCTIONS = Vec::new();

    loop 
    {
        let OPCODE = match if IS_LITTLE_ENDIAN 
        {
            SOURCE_ENDIAN::READ_LE::<R, u16>(READER)
        } 
        else 
        {
            SOURCE_ENDIAN::READ_BE::<R, u16>(READER)
        } 
        {
            Ok(OPCODE) => OPCODE,
            Err(ENDIAN_READ_ERR::EndOfFile) => break, 
            Err(e) => return Err(e),
        };

    
        let OPERAND = match if IS_LITTLE_ENDIAN 
        {
            SOURCE_ENDIAN::READ_LE::<R, u16>(READER)
        } 
        else 
        {
            SOURCE_ENDIAN::READ_BE::<R, u16>(READER)
        } 
        {
            Ok(OPERAND) => OPERAND,
            Err(ENDIAN_READ_ERR::EndOfFile) => 
            {
                eprintln!("Warning: Incomplete instruction found at end of file. OPCODE: 0x{:04X}", OPCODE);
                break; 
            }
            Err(e) => return Err(e),
        };

        INSTRUCTIONS.push(INSTR { OPCODE, OPERAND });
    }

    if INSTRUCTIONS.is_empty() {
        eprintln!("Warning: No valid instructions found in file.");
    }

    Ok(INSTRUCTIONS)
}

// FUNCTION TO PROCESS A BINARY FILE

fn PROC_FILE(F_PATH: &str, IS_LITTLE_ENDIAN: bool) -> Result<(), Box<dyn std::error::Error>> 
{
    let FILE = File::open(F_PATH)?;
    let METADATA = FILE.metadata()?;

    println!("Processing file: {}", F_PATH);
    println!("File size: {} bytes", METADATA.len());

    if METADATA.len() < std::mem::size_of::<u16>() as u64 
    {
        eprintln!(
            "Warning: File is smaller than the size of a single instruction (4 bytes). Partial data will be processed."
        );
    } 
    else if METADATA.len() % std::mem::size_of::<INSTR>() as u64 != 0 
    {
        eprintln!
        (
            "Warning: File size ({}) is not a multiple of instruction size (4 bytes). Some data might be ignored.",
            METADATA.len()
        );
    }

    let mut READER = BufReader::new(FILE);

    match PARSE_BINARY(&mut READER, IS_LITTLE_ENDIAN) 
    {
        Ok(INSTRUCTIONS) => 
        {
            if INSTRUCTIONS.is_empty() 
            {
                println!("File was empty or contained no valid instructions.");
            } 
            else 
            {
                println!("Found {} instructions:", INSTRUCTIONS.len());
                for (i, INSTR) in INSTRUCTIONS.iter().enumerate() 
                {
                    println!
                    (
                        "Instruction {}: OPCODE: 0x{:04X}, OPERAND: 0x{:04X}",
                        i, INSTR.OPCODE, INSTR.OPERAND
                    );
                }
            }
        }
        Err(e) => match e 
        {
            ENDIAN_READ_ERR::EndOfFile => 
            {
                eprintln!("Warning: File ended unexpectedly while parsing.");
            }
            ENDIAN_READ_ERR::IoError(err) => 
            {
                eprintln!("IO Error: {}", err);
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    eprintln!("The file might be smaller than expected or corrupted.");
                }
            }
            _ => return Err(Box::new(e)),
        },
    }

    Ok(())
}

fn main() 
{
    println!("HARRY CLARK - ENDIAN PARSER AND LEXER\n");
    let ARGS: Vec<String> = ENV::args().collect();

    if ARGS.len() < 3 {
        eprintln!("Usage: {} <INPUT_FILE> <ENDIANNESS>\n", ARGS[0]);
        eprintln!("ENDIANNESS: LE (little-endian) or BE (big-endian)");
        PROC::exit(1);
    }

    let INPUT = &ARGS[1];
    let ENDIANNESS = &ARGS[2];

    let IS_LITTLE_ENDIAN = match ENDIANNESS.as_str() 
    {
        "LE" => true,
        "BE" => false,
        _ => 
        {
            eprintln!("Invalid endianness. Use 'LE' for little-endian or 'BE' for big-endian.");
            PROC::exit(1);
        }
    };

    if let Err(ERR) = PROC_FILE(INPUT, IS_LITTLE_ENDIAN) 
    {
        eprintln!("Error processing file: {}", ERR);
        PROC::exit(1);
    }
}
