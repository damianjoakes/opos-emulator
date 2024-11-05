//! Module containing code related to interpreting OPOS firmware calls to the printer.
//! This is set up according to Epson's standards in their documentation,
//! located [here](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/).

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use std::ops::Deref;
use crate::interop::opos::{ESC, EXCLAMATION, MINUS, SP};

pub trait Interpret {
    fn interpret<T>(value: Box<dyn Interpretable>) -> T;
}

/// A trait describing a struct that may be interpreted (or *"deserialized"* from a set of bytes
/// that have been passed in from a buffer. Structs that this is implemented on should typically
/// include some way of storing and accessing assigned bytes.
pub trait Interpretable: InterpretableClone + Debug {
    /// Accepts two arguments which are the current byte and the next byte with a `ByteInterpreter`.
    ///
    /// `next` is always an optional piece of data, as it's possible for the `ByteInterpreter` to
    /// hit the end of a byte sequence. `next` will only ever be `None` if `current` is the last
    /// byte in the sequence.
    ///
    /// Used for assigning passed in bytes. This should return a boolean on whether the provided
    /// data was relevant or not. The moment the data is not relevant, any `ByteInterpreter` will
    /// stop interpreting for the command.
    fn assign_bytes(&mut self, current: u8, next: Option<u8>) -> bool;

    /// Returns whether the current `Interpretable` is valid and complete. If this returns `false`,
    /// then it the constructed `Interpretable` will not be constructed from the buffer
    /// interpreter.
    fn is_valid(&self) -> bool;
}

pub trait InterpretableClone {
    fn clone_box(&self) -> Box<dyn Interpretable>;
}

impl<T> InterpretableClone for T where T: 'static + Interpretable + Clone {
    fn clone_box(&self) -> Box<dyn Interpretable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Interpretable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Struct which wraps a buffer, and allows for translating the data into instruction sets.
pub struct ByteInterpreter<'a> {
    buffer: &'a [u8],
    interpretables: BTreeMap<u8, Box<dyn Interpretable>>
}

impl<'a> ByteInterpreter<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            interpretables: BTreeMap::<u8, Box<dyn Interpretable>>::new()
        }
    }

    /// Adds an `Interpretable` into the internal `interpretables` map. The provided `key` is used
    /// to store the `Interpretable`. This `key` should be the decimal mapping of the command
    /// in OPOS (these are provided as `const`s in `interop::opos`).
    pub fn add_interpretable(&mut self, key: u8, interpretable: Box<dyn Interpretable>) {
        self.interpretables.insert(key, interpretable);
    }

    /// Interprets the internally stored buffer. Runs all (viable) bytes against the internal
    /// `interpretables` map. This consumes `self.buffer`, meaning the buffer will need to be
    /// reassigned after this is called.
    pub fn interpret_buffer(&mut self) -> Result<Vec<Box<dyn Interpretable>>, Box<dyn Error>> {
        let buffer = self.buffer;
        let mut buf_iter = buffer.iter().peekable();

        let mut interpretations = Vec::new();

        while let Some(byte) = buf_iter.next() {
            let inter = self.interpretables.get_mut(byte);

            // Check if our `interpretables` includes the current byte. If it does,
            // keep reading from the buffer until the `Interpretable` states that it no
            // longer should be consuming data.
            if let Some(interpretable) = inter {
                let mut saved_iter = buf_iter.clone();

                'inner: loop {
                    saved_iter = buf_iter.clone();

                    let current = match buf_iter.next() {
                        None => { break 'inner; }
                        Some(v) => { v }
                    };

                    let next = match buf_iter.next() {
                        None => { None }
                        Some(v) => { Some(*v) }
                    };

                    if !interpretable.assign_bytes(*current, next) {
                        if interpretable.is_valid() {
                            let i = interpretable.clone();
                            interpretations.push(i);
                        }

                        buf_iter = saved_iter;
                        break 'inner;
                    }
                }
            }
        }

        Ok(interpretations)
    }
}

#[derive(Clone, Debug)]
pub struct EscHandler {
    subcommand: u8,
    args: Vec<u8>
}

impl EscHandler {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            subcommand: 0u8
        }
    }
}

impl Interpretable for EscHandler {
    fn assign_bytes(&mut self, current: u8, next: Option<u8>) -> bool {
        if current == ESC {
            let next = match next {
                None => { return true; }
                Some(sub) => {
                    sub
                }
            };

            if matches!(next, EXCLAMATION | SP | MINUS) {
                true
            } else {
                false
            }
        } else if matches!(current, EXCLAMATION | SP | MINUS) {
            let next = match next {
                None => { return true; }
                Some(sub) => {
                    sub
                }
            };

            self.subcommand = current;
            self.args.push(next);
            false
        } else {
            false
        }
    }

    fn is_valid(&self) -> bool {
        self.subcommand != 0u8
    }
}