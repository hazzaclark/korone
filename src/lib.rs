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

