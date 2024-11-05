//! Contains built in interpretables for interpreting OPOS buffers.

use crate::core::interpretables::Interpret;
use crate::interop::opos::*;

/// Interpreter for `ESC` Character commands.
/// For more information, click
/// [here](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/esc_space.html).
#[derive(Clone, Debug)]
pub struct EscInterpreter {
    counted_args: usize,
    arg_count: usize,
    subcommand: u8,
    args: Vec<u8>,
}

impl EscInterpreter {
    /// Constructs a new `EscInterpreter`.
    pub fn new() -> Self {
        Self {
            counted_args: 0,
            arg_count: 1,
            args: Vec::new(),
            subcommand: 0u8,
        }
    }

    /// Checks to see if the supplied subcommand matches the list of valid subcommands defined for
    /// `ESC` character commands.
    pub fn is_valid_subcommand(&self, value: &u8) -> bool {
        matches!(*value,
                EXCLAMATION
                | SP
                | MINUS
                | UPPER_M
                | UPPER_G
                | UPPER_E
                | UPPER_R
                | UPPER_V
                | LOWER_R
                | LOWER_T
                | OPEN_CURLY
            )
    }
}

impl crate::core::interpretables::Interpretable for EscInterpreter {
    fn should_continue(&self, value: &u8) -> bool {
        if self.is_valid_subcommand(value) {
            true
        } else {
            if self.counted_args < self.arg_count {
                true
            } else {
                // Note that `ESC` is also caught here is false. This is because at this point,
                // `ESC` should be handled as a normal value since we're already in the middle of
                // interpreting an `ESC` command.
                false
            }
        }
    }

    fn assign(&mut self, value: u8) {
        // If the current value is `ESC`, then it means that we've read at the beginning of the
        // command and don't need to continue.
        if value == ESC {
            return;
        } else {
            // Check if the provided value is a valid subcommand. If it is, and we haven't assigned
            // a subcommand yet, then assign the subcommand to self. Otherwise, check if we've
            // assigned any arguments yet. If so, then instead check if the provided value needs
            // to be pushed into our arguments.
            if self.subcommand == 0u8 {
                if self.is_valid_subcommand(&value) {
                    self.subcommand = value;
                }
            } else {
                if self.counted_args < self.arg_count {
                    self.args.push(value);
                    self.counted_args += 1;
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        // If we've filled our arguments, and our subcommand is not zero, then this interpretable
        // has finished.
        (self.counted_args == self.arg_count) && (self.subcommand != 0u8)
    }
}

/// Interpreter for `GS` Character commands. For more information, click
/// [here](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/gs_exclamation.html).
#[derive(Clone, Debug)]
pub struct GsInterpreter {
    counted_args: usize,
    arg_count: usize,
    subcommand: u8,
    args: Vec<u8>,
}


impl GsInterpreter {
    /// Constructs a new `EscInterpreter`.
    pub fn new() -> Self {
        Self {
            counted_args: 0,
            arg_count: 1,
            args: Vec::new(),
            subcommand: 0u8,
        }
    }

    /// Checks to see if the supplied subcommand matches the list of valid subcommands defined for
    /// `ESC` character commands.
    pub fn is_valid_subcommand(&self, value: &u8) -> bool {
        matches!(*value, EXCLAMATION | UPPER_B | LOWER_B)
    }
}

impl crate::core::interpretables::Interpretable for GsInterpreter {
    fn should_continue(&self, value: &u8) -> bool {
        if self.is_valid_subcommand(value) {
            true
        } else {
            if self.counted_args < self.arg_count {
                true
            } else {
                false
            }
        }
    }

    fn assign(&mut self, value: u8) {
        if value == GS {
            return;
        } else {
            if self.subcommand == 0u8 {
                if self.is_valid_subcommand(&value) {
                    self.subcommand = value;
                }
            } else {
                if self.counted_args < self.arg_count {
                    self.args.push(value);
                    self.counted_args += 1;
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        // If we've filled our arguments, and our subcommand is not zero, then this interpretable
        // has finished.
        (self.counted_args == self.arg_count) && (self.subcommand != 0u8)
    }
}

impl Interpret for GsInterpreter {
    fn interpret<T>(self) -> T {
        todo!()
    }
}