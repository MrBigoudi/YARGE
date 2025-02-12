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

    /// Thrown when an initialization failed
    InitializationFailure,

    /// Thrown when a shut down failed
    ShutDownFailure,

    /// Thrown when a platform dependent error occurs
    PlatformDependentFailure,
    
}
