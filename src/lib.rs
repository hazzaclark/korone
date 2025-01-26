// COPYRIGHT (C) HARRY CLARK 2025

// LIGHTWEIGHT ENDIANESS PARSER AND LEXER IN RUST

// THE FOLLOWING FILE PERTAINS TOWARDS THE SINGLE HEADER FUNCTIONALITY ENCOMPASSING
// THIS PROJECT - PROVIDING DECLARATIONS AND THE LIKE FOR THE BASE IO

// SYSTEM INCLUDES

use std::mem;
use std::marker::PhantomData;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};

// STATICALLY AVAILABLE TYPE LEVEL TRAITS FOR ASSERTING
// DIFFERENT GATE CONDITIONS

mod TYPE_TRAIT
{
    pub trait NOT { type Output; }

    impl NOT for bool { type Output = bool; }

    pub trait Conjunction<T: = bool> { type Output; }

    pub trait Disconjunction<T: = bool> { type Output; }

    pub trait IsAnyOf<T> { const VALUE: bool; }

    pub trait TRAIT_CONT 
    {
        fn HAS_PUSH_BACK() -> bool;
        fn HAS_SIZE() -> bool;
        fn HAS_RESIZE() -> bool;
        fn HAS_DATA() -> bool;
    }
}

// ENDIANESS DEFINES AND DIRECTIVES

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait ENDIAN_BYTE_SWAP
{
    fn SWAP_BYTES(self) -> Self;
}

// FULL FLEDGED ENDIAN UTILITIES USED FOR CONVERSION AND SWAPPING
// THE FOLLOWING WILL TAKE A LOOK INTO THE VARIOUS TYPES ENCOMPASSING SIGNED 
// AND UNSIGNED VALUES AND HANDLE THEIR RESPECTIVE LENGTHS

pub struct ENDIAN_UTIL;

impl ENDIAN_UTIL
{
    pub fn B_TYPE_SWAP<T: ENDIAN_BYTE_SWAP>(VALUE: T) -> T
    {
        VALUE.SWAP_BYTES()
    }

    pub fn SELECT_UNSIGNED<T>(SIZE: usize) -> Option<T>
    where
        T: From<u8> + From<u16> + From<u32> + From<u64>
    {

        // MATCH SIZE BASED ON AMOUNT OF BYTES BEING ALLOCATED FOR EACH DATA TYPE

        match SIZE
        {
            1 => Some(T::from(0u8)),
            2 => Some(T::from(0u16)),
            4 => Some(T::from(0u32)),
            8 => Some(T::from(0u64)),
            _ => None
        }
    }
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
