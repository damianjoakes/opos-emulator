//! Module containing code related to interpreting OPOS firmware calls to the printer.
//! This is set up according to Epson's standards in their documentation,
//! located [here](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/).

pub mod built_in;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;

/// A trait used on `Interpretable`s as a way to extract readable data from them, or do some other
/// command execution.
///
/// This is not yet implemented fully, but the end goal is to use this to write data to an image
/// stream or some display method for viewing the receipts that have been printed.
///
/// #todo
pub trait Interpret: Interpretable {
    /// Converts `self` into some interpreted value, consuming `self`.
    fn interpret<T>(self) -> T;
}

/// A trait describing a struct that may be interpreted (or *"deserialized"* from a set of bytes
/// that have been passed in from a buffer. Structs that this is implemented on should typically
/// include some way of storing and accessing assigned bytes.
pub trait Interpretable: InterpretableClone + Debug {
    /// Returns a boolean stating whether the `Interpretable` should continue claiming bytes when
    /// given `value`.
    fn should_continue(&self, value: &u8) -> bool;

    /// Assigns the provided value to the `Interpretable` in some way. This logic is stored
    /// entirely in the trait object.
    fn assign(&mut self, value: u8);

    /// Returns whether the current `Interpretable` is valid. If this returns `false`,
    /// then the `Interpretable` will not be constructed from the buffer interpreter.
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
                let mut interpretable = Box::clone(interpretable);

                'inner: loop {
                    let peeked = match buf_iter.peek() {
                        None => { break 'inner; }
                        Some(v) => { v }
                    };

                    if !interpretable.should_continue(*peeked) {
                        if interpretable.is_valid() {
                            let i = interpretable.clone();
                            interpretations.push(i);
                        }

                        break 'inner;
                    } else {
                        let current = match buf_iter.next() {
                            None => { break 'inner; }
                            Some(v) => { *v }
                        };

                        interpretable.assign(current);
                        continue 'inner;
                    }
                }
            }
        }

        Ok(interpretations)
    }
}