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

    /// Thrown when a Vulkan issue happens
    VulkanError,

    /// Thrown when an IO error occured
    IO,

    /// Thrown when something is not yet implemented
    NotImplemented,

    /// Thrown when an invalid index is used inside a data structure
    InvalidIndex,

    /// Thrown when trying to duplicate a value unexpectedly
    Duplicate,

    /// Thrown when a bad request is attempted
    BadRequest,

    /// Thrown when something is not supported
    NotSupported,
}
