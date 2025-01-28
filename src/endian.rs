// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

// THE FOLLOWING FILE PERTAINS TOWARDS THE SINGLE HEADER FUNCTIONALITY ENCOMPASSING
// THIS PROJECT - PROVIDING DECLARATIONS AND THE LIKE FOR THE BASE IO

// SYSTEM INCLUDES

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::Read;

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
            ENDIAN_READ_ERR::BufferTooSmall => write!(f, "Buffer is too small to process the data."),
            ENDIAN_READ_ERR::ConversionError => write!(f, "Error during byte conversion."),
            ENDIAN_READ_ERR::EndOfFile => write!(f, "Reached end of file."),
        }
    }
}

impl std::error::Error for ENDIAN_READ_ERR {}

pub struct SOURCE_ENDIAN;
impl SOURCE_ENDIAN 
{
    // DEFINE A GENERIC METHOD TO READ A SPECIFIED VALUE FROM THE READER
    // THE DEFINING FEATURE IS THAT THIS READ SCHEMA MUST IMPLEMENT THE READ TRAIT
    // WHICH ENCOMPASSES A FILE, BUFFER, OR STREAM


    pub fn READ_LE<R: Read, T>(READER: &mut R) -> Result<T, ENDIAN_READ_ERR>
    where
        T: Default + Copy + std::fmt::Debug + Sized,
    {
        let SIZE = std::mem::size_of::<T>();
        let mut BUFFER = vec![0u8; SIZE];

        match READER.read_exact(&mut BUFFER) 
        {
            Ok(_) => 
            {
                let mut VALUE = T::default();
                for (i, &BYTE) in BUFFER.iter().enumerate() 
                {
                    unsafe 
                    {
                        let VALUE_PTR = (&mut VALUE as *mut T as *mut u8).add(i);
                        *VALUE_PTR = BYTE;
                    }
                }
                Ok(VALUE)
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => 
            {
                Err(ENDIAN_READ_ERR::EndOfFile)
            }
            
            Err(err) => Err(ENDIAN_READ_ERR::IoError(err)),
        }
    }

    pub fn READ_BE<R: Read, T>(READER: &mut R) -> Result<T, ENDIAN_READ_ERR>
    where
        T: Default + Copy + std::fmt::Debug + Sized,
    {
        let SIZE = std::mem::size_of::<T>();
        let mut BUFFER = vec![0u8; SIZE];

        match READER.read_exact(&mut BUFFER) 
        {
            Ok(_) => 
            {
                let mut VALUE = T::default();
                for (i, &BYTE) in BUFFER.iter().rev().enumerate() 
                {
                    unsafe 
                    {
                        let VALUE_PTR = (&mut VALUE as *mut T as *mut u8).add(i);
                        *VALUE_PTR = BYTE;
                    }
                }
                Ok(VALUE)
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => 
            {
                Err(ENDIAN_READ_ERR::EndOfFile)
            }
            Err(err) => Err(ENDIAN_READ_ERR::IoError(err)),
        }
    }
}
