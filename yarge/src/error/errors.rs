/// The errors levels
#[derive(Debug)]
pub enum ErrorLevel {
    /// Error that can't be recovered    
    Error,
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
    /// An unrecognized error type
    Unknown,

    /// Thrown when trying to divide by 0
    DivisionByZero,

    /// Thrown when a wrong argument is given to a function
    /// Take as parameter the expected argument
    WrongArgument(String),

    /// Thrown when an element does not exist
    DoesNotExist,

    /// Thrown when a vulkan issue happens
    VulkanError,

    /// Thrown when an IO error occured
    IO,

    /// Thrown when something is not yet implemented
    NotImplemented,
}
