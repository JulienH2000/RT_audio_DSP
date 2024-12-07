use std::error::Error;
use std::fmt;

// Audio Engine Error

#[derive(Debug)]
pub struct AudioError {
    pub message: String,
}

impl AudioError {
    pub fn new_msg(message: &str) -> AudioError {
        AudioError {
            message: message.to_string(),
        }
    }

    pub fn new(message: &str) -> AudioError {
        AudioError {
            message: message.to_string(),
        }
    }
}

impl Error for AudioError {}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for AudioError {
    fn from(error: std::io::Error) -> Self {
        AudioError {
            message: format!("An audio engine error occurred: {}", error),
        }
    }
}


// Hardware Errors

#[derive(Debug)]
pub struct HardwareError {
    pub message: String,
}

impl HardwareError {
    pub fn new(message: &str) -> HardwareError {
        HardwareError {
            message: message.to_string(),
        }
    }
}

impl Error for HardwareError {}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for HardwareError {
    fn from(error: std::io::Error) -> Self {
        HardwareError {
            message: format!("An hardware error occurred: {}", error),
        }
    }
}

#[derive(Debug)]
pub struct RunningError {}

impl RunningError {
    pub fn new_msg() -> RunningError {
        RunningError {}
    }

    pub fn new() -> RunningError {
        RunningError {}
    }
}

impl Error for RunningError {}

impl fmt::Display for RunningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Running Error !")
    }
}

#[derive(Debug)]
pub enum HttpError {
    InvalidMethod,
    UnsupportedReq
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMethod => write!(f, "Invalid HTTP Request Method"),      
            Self::UnsupportedReq => write!(f, "Unsupported HTTP Request"),      
        }
    }
}

impl Error for HttpError {}


#[derive(Debug)]
pub struct CmdError {}

impl Error for CmdError {}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown Command")
    }
}
