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

    pub fn READ<R: Read, T>(READER: &mut R) -> Result<T, ENDIAN_READ_ERR>
    where
        T: Default + Copy
    {
        let mut VALUE = T::default();

        // USE THE READ_EXACT METHOD IN ORDE3R TO EXTRAPOLATE THE SIZE OF
        // BYTES INTO VALUE

        // I LEARNT THAT USING THE FROM RAW PARTS MUTABLE DIRECTIVE ALLOWS
        // FOR THE MUTABLE ALLOCATION OF MEMORY TO THAT POINTER

        READER.read_exact(unsafe 
        {
            std::slice::from_raw_parts_mut
            (
                &mut VALUE as *mut T as *mut u8,
                std::mem::size_of::<T>(),
            )
        }).map_err(ENDIAN_READ_ERR::IoError)?;

        Ok(VALUE)
    }
}
