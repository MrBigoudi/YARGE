#![warn(missing_docs)]

//! Library containing everything needed to debug the code

/// The errors levels
#[derive(Debug)]
pub enum ErrorLevel {
    /// Error that can't be recovered    
    Fatal,
    /// Error that can be recovered
    Warn,
    /// Debug information
    Debug,
    /// Other printable information
    Info,
}

/// The errors types
#[derive(Debug)]
pub enum ErrorType {
    /// Thrown when trying to divide by 0
    DivisionByZero,
}
