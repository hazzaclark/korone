// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

// THE FOLLOWING FILE PERTAINS TOWARDS THE SINGLE HEADER FUNCTIONALITY ENCOMPASSING
// THIS PROJECT - PROVIDING DECLARATIONS AND THE LIKE FOR THE BASE IO

// SYSTEM INCLUDES

use std::cmp;
use std::array;
use std::mem;
use std::io;
use std::marker::PhantomData;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};

pub enum ENDIANESS
{
    Little,
    Big,
}

pub enum ENDIAN_READ_ERR 
{
    IoError(std::io::Error),
    BufferTooSmall,
    ConversionError,
}

pub trait ENDIAN_READ<T>
{
    fn READ_ENDIAN(&mut self, E: ENDIANESS) -> Result<T, ENDIAN_READ_ERR>;
}
