//! Allows encoding a struct implementing serde's Serialize using rustc_serialize, or vice versa.
//!
//! This is now without difficulties, however.
//! For one, there is another step between your struct and your byte stream.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications,
        warnings)]

#[cfg(test)]
mod test;

extern crate bincode;
extern crate either;
extern crate serde;
extern crate rustc_serialize;

use bincode::rustc_serialize::{DecodingError, decode, encode};
use bincode::serde::{DeserializeError, SerializeError, deserialize, serialize};
use bincode::SizeLimit;
use either::Either;
use rustc_serialize::{Encodable, Decodable};
use serde::{Deserialize, Serialize};
use serde::de::value::Error as SerdeError;

/// The serialization mid-end the Interop struct was originally made from.
#[derive(Serialize, Deserialize, RustcDecodable, RustcEncodable, Clone, Copy, Debug)]
pub enum Origin {
    /// The Interop was constructed using serde.
    Serde,
    /// The interop was constructed using rustc-serialize.
    RustcSerialize,
}

/// Represents an error that hapened during serialization/deserialization.
#[derive(Debug)]
pub enum Error {
    /// A custom error message.
    Custom(String),
    /// An internal serde error.
    Serde(SerdeError),
    /// The encoding of the value was incorrect.
    ///
    /// This contains a DeserializeError from bincode, because the struct in bincode that normally represents this value is private.
    /// It is however, gaurenteed that this will only ever contain DeserializeError::InvalidEncoding.
    InvalidEncoding(Either<DeserializeError, DecodingError>),
}

impl From<SerializeError> for Error {
    fn from(from: SerializeError) -> Self {
        match from {
            SerializeError::Custom(err) => Error::Custom(err),
            SerializeError::IoError(_) => panic!("Attempt to convert SerializeError::IoError to Error"),
            SerializeError::SizeLimit => panic!("Attempt to convert SerializeError::SizeLimit to Error"),
        }
    }
}

impl From<DeserializeError> for Error {
    fn from(from: DeserializeError) -> Self {
        match from {
            DeserializeError::IoError(_) => panic!("Attempt to convert SerializeError::IoError to Error"),
            DeserializeError::InvalidEncoding(_) => Error::InvalidEncoding(Either::Left(from)),
            DeserializeError::SizeLimit => panic!("Attempt to convert SerializeError::SizeLimit to Error"),
            DeserializeError::Serde(err) => Error::Serde(err),
        }
    }
}

impl From<DecodingError> for Error {
    fn from(from: DecodingError) -> Self {
        match from {
            DecodingError::InvalidEncoding(_) => Error::InvalidEncoding(Either::Right(from)),
            DecodingError::IoError(_) => panic!("Attempt to convert DecodingError::IoError to Error"),
            DecodingError::SizeLimit => panic!("Attempt to convert DecodingError::SizeLimit to Error"),
        }
    }
}

/// Allows encoding a struct implementing serde's Serialize using rustc_serialize, or vice versa.
#[derive(Serialize, Deserialize, RustcDecodable, RustcEncodable, Debug)]
pub struct Interop {
    repr: Vec<u8>,
    /// The serialization mid-end the Interop struct was originally made from.
    pub origin: Origin,
}

impl Interop {
    /// Creates an Interop struct using serde.
    pub fn serde<T: Serialize>(from: &T) -> Result<Self, Error> {
        Ok(Interop {
            repr: try!(serialize(from, SizeLimit::Infinite)),
            origin: Origin::Serde,
        })
    }

    /// Deserializes the Interop into it's origin value.
    ///
    /// # Panics
    /// * Calling on a Interop that was not constructed using the serde method.
    pub fn serde_deser<T: Deserialize>(&self) -> Result<T, Error> {
        Ok(match self.origin {
            Origin::Serde => try!(deserialize(&self.repr)),
            Origin::RustcSerialize => panic!("Called serde_deser on value constructed using rustc-serialize"),
        })
    }

    /// Creates an Interop struct using rustc-serialize.
    pub fn rustc<T: Encodable>(from: &T) -> Result<Self, Error> {
        Ok(Interop {
            repr: encode(from, SizeLimit::Infinite).expect("Assertion error: encode returned err value with SizeLimit::Infinite"),
            origin: Origin::RustcSerialize,
        })
    }

    /// Deserializes the Interop into it's origin value.
    ///
    /// # Panics
    /// * Calling on a Interop that was not constructed using the rustc-serialize method.
    pub fn rustc_deser<T: Decodable>(&self) -> Result<T, Error> {
        Ok(match self.origin {
            Origin::Serde => panic!("Called rustc_deser on value constructed using serde"),
            Origin::RustcSerialize => try!(decode(&self.repr)),
        })
    }
}
