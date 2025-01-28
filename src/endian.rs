// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

// THE FOLLOWING FILE PERTAINS TOWARDS THE SINGLE HEADER FUNCTIONALITY ENCOMPASSING
// THIS PROJECT - PROVIDING DECLARATIONS AND THE LIKE FOR THE BASE IO

// SYSTEM INCLUDES

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::{Read};

#[derive(Debug)]
pub enum ENDIAN_READ_ERR 
{
    IoError(std::io::Error),
    BufferTooSmall,
    ConversionError,
    EndOfFile,
}

// CREATE A STD DISPLAY OUTPUT FOR ANY AND ALL OF THE POSSIBLE OPTIONS

impl std::fmt::Display for ENDIAN_READ_ERR 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            ENDIAN_READ_ERR::IoError(err) => write!(f, "IO Error: {}", err),
            ENDIAN_READ_ERR::BufferTooSmall => write!(f, "Buffer is too small"),
            ENDIAN_READ_ERR::ConversionError => write!(f, "Conversion error"),
            ENDIAN_READ_ERR::EndOfFile => write!(f, "End of file reached"),
        }
    }
}

impl std::error::Error for ENDIAN_READ_ERR {}


// MAIN ENCOMPASSING READING AND WRITING SCHEMA

pub struct SOURCE_ENDIAN;
impl SOURCE_ENDIAN
{
    // DEFINE A GENERIC METHOD TO READ A SPECIFIED VALUE FROM THE READER
    // THE DEFINING FEATURE IS THAT THIS READ SCHEMA MUST IMPLEMENT THE READ TRAIT
    // WHICH ENCOMPASSES A FILE, BUFFER, OR STREAM

    // NOWE WE CAN READ SPECIFIC ENDIAN LEVELS

    pub fn READ_LE<R: Read, T>(READER: &mut R) -> Result<T, ENDIAN_READ_ERR>
    where
        T: Default + Copy + std::fmt::Debug,
        {
            let mut VALUE = T::default();
            let SIZE = std::mem::size_of::<T>();
            let mut BUFFER = vec![0u8; SIZE];

            READER.read_exact(&mut BUFFER).map_err(ENDIAN_READ_ERR::IoError)?;

            // CONVERT LE BYTES TO THE TARGET

            for(i, BYTE) in BUFFER.iter().enumerate()
            {
                unsafe
                {
                    let VALUE_PTR = (&mut VALUE as *mut T as *mut u8).add(i);
                    *VALUE_PTR = *BYTE;
                }
            }

            Ok(VALUE)
        }


    pub fn READ_BE<R: Read, T>(READER: &mut R) -> Result<T, ENDIAN_READ_ERR>
    where
        T: Default + Copy + std::fmt::Debug,
        {
            let mut VALUE = T::default();
            let SIZE = std::mem::size_of::<T>();
            let mut BUFFER = vec![0u8; SIZE];
    
            READER.read_exact(&mut BUFFER).map_err(ENDIAN_READ_ERR::IoError)?;
    
            // CONVERT LE BYTES TO THE TARGET
    
            for(i, BYTE) in BUFFER.iter().rev().enumerate()
            {
                unsafe
                {
                    let VALUE_PTR = (&mut VALUE as *mut T as *mut u8).add(i);
                    *VALUE_PTR = *BYTE;
                }
            }
    
            Ok(VALUE)
        }
}
