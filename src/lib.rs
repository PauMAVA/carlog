//! `carlog` is a simple, lightweight crate that provides Cargo logging style messages via the
//! `Status` struct or via multiple macros that recreate common cargo message formats:
//! * Cargo ok: `carlog_ok!`
//! * Cargo info: `carlog_info!`
//! * Cargo warning: `carlog_warning!`
//! * Cargo error: `carlog_error!`
//!
//! The crate provides support for logging to both stdout and stderr and to any stream that implements
//! the `Write` trait.
//!
//! ## Example
//! ```ignore
//! #[macro_use] extern crate carlog;
//!
//! use carlog::prelude::*;
//!
//! let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
//! status.print_stdout("carlog v0.1.0");
//!
//! carlog_ok!("Compiled", "carlog v0.1.0");
//! ```
//! Output:
//! <div style="padding: 1%; padding-left: 50px; background-color: black;">
//!     <span style="color: #16C60C;"><b>Compiled</b></span><span> carlog v0.1.0</span>
//! </div>

use colored::*;
use std::io;
use std::io::{stderr, stdout, Write};

/// Module to import required structs and enums to use this crate.
///
/// ## Example
/// ```
/// use carlog::prelude::*;
/// ```
pub mod prelude {
    pub use crate::CargoColor;
    pub use crate::CarlogStream;
    pub use crate::Status;
}

/// Cargo terminal colors.
#[derive(Copy, Clone)]
pub enum CargoColor {
    Green,
    Cyan,
    Yellow,
    Red,
    White,
    Black,
}

impl Default for CargoColor {
    fn default() -> Self {
        Self::White
    }
}

/// Carlog library streams.
///
/// This enum contains the two output standard streams:
/// * stdout
/// * stderr
/// It also has a custom variant which holds a mutable reference to a writeable stream (socket, vec, slice...).
///
/// The default variant is the stdout.
///
/// ## Example
/// ```
/// use carlog::prelude::*;
///
/// let stdout = CarlogStream::Stdout;
/// let stderr = CarlogStream::Stderr;
/// let mut output = Vec::<u8>::new();
/// let custom = CarlogStream::Custom(&mut output);
/// ```
pub enum CarlogStream<'a> {
    Stdout,
    Stderr,
    Custom(&'a mut dyn Write),
}

impl Default for CarlogStream<'_> {
    fn default() -> Self {
        Self::Stdout
    }
}

/// Simple cargo status log.
///
/// This is the part displayed before the actual message to be logged i.e. 'Compiled'.
///
/// ## Example
/// ```
/// use carlog::prelude::*;
///
/// let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
/// status.print_stdout("carlog v0.1.0");
/// ```
#[derive(Default)]
pub struct Status {
    /// If the status must be padded to 12 characters to the right using spaces.
    justify: bool,

    /// If the status must be bold.
    bold: bool,

    /// The color of the status.
    color: CargoColor,

    /// The string of the status.
    status: String,
}

impl Status {
    /// Creates a new empty status.
    ///
    /// The status has the default values:
    /// * <b>NO</b> justification.
    /// * <b>NO</b> boldness.
    /// * White color.
    /// * Empty status string.
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Justify the status.
    ///
    /// Sets the status to be padded to 12 characters to the right using spaces.
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().justify();
    /// ```
    pub fn justify(mut self) -> Self {
        self.justify = true;
        self
    }

    /// Set the status to be bold.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().bold();
    /// ```
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set the color of the status.
    ///
    /// * `color`: The cargo color of the status.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().color(CargoColor::Cyan);
    /// ```
    pub fn color(mut self, color: CargoColor) -> Self {
        self.color = color;
        self
    }

    /// Set the string status.
    ///
    /// * `str`: The status text from a type that can be converted to a string reference.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().status("Text");
    /// ```
    pub fn status<S>(mut self, str: S) -> Self
    where
        S: AsRef<str>,
    {
        self.status = str.as_ref().to_string();
        self
    }

    /// Print the status to stdout.
    ///
    /// `msg`: The message to be printed alongside the status.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
    /// status.print_stdout("carlog v0.1.0");
    /// ```
    pub fn print_stdout<S>(self, msg: S) -> io::Result<()>
    where
        S: AsRef<str>,
    {
        self.print(stdout().lock(), msg)
    }

    /// Print the status to stderr.
    ///
    /// `msg`: The message to be printed alongside the status.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
    /// status.print_stderr("carlog v0.1.0");
    /// ```
    pub fn print_stderr<S>(self, msg: S) -> io::Result<()>
    where
        S: AsRef<str>,
    {
        self.print(stderr().lock(), msg)
    }

    /// Print the status to the specified stream.
    ///
    /// `stream`: The stream where the status and message will be written.
    /// `msg`: The message to be printed alongside the status.
    ///
    /// ## Example
    /// ```
    /// use carlog::prelude::*;
    ///
    /// let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
    /// let mut output = Vec::<u8>::new();
    /// status.print(output, "carlog v0.1.0");
    /// ```
    pub fn print<W, S>(self, mut stream: W, msg: S) -> io::Result<()>
    where
        W: Write,
        S: AsRef<str>,
    {
        let status = Self::color_str(self.color, self.bold, &self.status);
        if self.justify {
            let padding = " ".repeat(usize::saturating_sub(12, self.status.len()));
            write!(stream, "{}{}", padding, status)?;
        } else {
            write!(stream, "{}", status)?;
        }
        writeln!(stream, "{}", msg.as_ref())?;
        stream.flush()?;
        Ok(())
    }

    fn color_str<S>(color: CargoColor, bold: bool, str: S) -> String
    where
        S: AsRef<str>,
    {
        let mut colored = match color {
            CargoColor::Green => str.as_ref().green(),
            CargoColor::Cyan => str.as_ref().cyan(),
            CargoColor::Yellow => str.as_ref().bright_yellow(),
            CargoColor::Red => str.as_ref().bright_red(),
            CargoColor::White => str.as_ref().white(),
            CargoColor::Black => str.as_ref().black(),
        };
        if bold {
            colored = colored.bold()
        }
        colored.to_string()
    }
}

/// Print a cargo like message.
///
/// ## Example
/// ```ignore
/// #[macro_use] extern crate carlog;
///
/// use carlog::prelude::*;
///
/// carlog!("Compiling", "carlog v0.1.0"); // Not justified, not bold, stdout, white.
/// carlog!("Compiling", "carlog v0.1.0", CargoColor::Cyan); // Not justified, not bold, stdout.
///
/// let mut output = Vec::<u8>::new();
/// carlog!(
///     "Compiling",                      // Status text
///     "carlog v0.1.0",                  // Message
///     true,                             // Bold
///     false,                            // Justified
///     CargoColor::Cyan,                 // Color
///     CarlogStream::Custom(&mut output) // Stream
/// );
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[macro_export]
macro_rules! carlog {
    ($status:expr, $message:expr) => {
        carlog!($status, $message, crate::CargoColor::default());
    };
    ($status:expr, $message:expr, $color:expr) => {
        carlog!(
            $status,
            $message,
            false,
            false,
            $color,
            crate::CarlogStream::default()
        )
    };
    ($status:expr, $message:expr, $bold:expr, $justify:expr, $color:expr, $stream:expr) => {
        let mut status = crate::Status::new().color($color).status($status);
        if $bold {
            status = status.bold();
        }
        if $justify {
            status = status.justify();
        }
        match $stream {
            crate::CarlogStream::Stdout => status
                .print_stdout($message)
                .expect("Failed to print to stdout!"),
            crate::CarlogStream::Stderr => status
                .print_stderr($message)
                .expect("Failed to print to stderr!"),
            crate::CarlogStream::Custom(stream) => status
                .print(stream, $message)
                .expect("Failed to print to custom stream!"),
        }
    };
}

/// Print an info-like cargo message.
///
/// The status is justified, bold and in cyan.
///
/// ## Example
/// ```ignore
/// #[macro_use] extern crate carlog;
///
/// use carlog::prelude::*;
///
/// carlog_info!("Compiling", "carlog v0.1.0");
/// let mut output = Vec::<u8>::new();
/// carlog_info!("Compiling", "carlog v0.1.0", CarlogStream::Custom(&mut output));
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[macro_export]
macro_rules! carlog_info {
    ($status:expr, $message:expr) => {
        carlog_info!($status, $message, crate::CarlogStream::default());
    };
    ($status:expr, $message:expr, $stream:expr) => {
        carlog!(
            $status,
            format!(" {}", $message),
            true,
            true,
            crate::CargoColor::Cyan,
            $stream
        );
    };
}

/// Print an ok-like cargo message.
///
/// The status is justified, bold and in green.
///
/// ## Example
/// ```ignore
/// #[macro_use] extern crate carlog;
///
/// use carlog::prelude::*;
///
/// carlog_ok!("Compiled", "carlog v0.1.0");
/// let mut output = Vec::<u8>::new();
/// carlog_ok!("Compiled", "carlog v0.1.0", CarlogStream::Custom(&mut output));
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[macro_export]
macro_rules! carlog_ok {
    ($status:expr, $message:expr) => {
        carlog_ok!($status, $message, crate::CarlogStream::default());
    };
    ($status:expr, $message:expr, $stream:expr) => {
        carlog!(
            $status,
            format!(" {}", $message),
            true,
            true,
            crate::CargoColor::Green,
            $stream
        );
    };
}

/// Print an warning like cargo message.
///
/// The status is not justified, not bold and light yellow with the status text 'warning'.
///
/// ## Example
/// ```ignore
/// #[macro_use] extern crate carlog;
///
/// use carlog::prelude::*;
///
/// carlog_warning!("carlog (v0.1.0) generated a warning!");
/// let mut output = Vec::<u8>::new();
/// carlog_warning!("carlog (v0.1.0) generated a warning!", CarlogStream::Custom(&mut output));
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[macro_export]
macro_rules! carlog_warning {
    ($message:expr) => {
        carlog_warning!($message, crate::CarlogStream::default());
    };
    ($message:expr, $stream:expr) => {
        carlog!(
            "warning",
            format!(": {}", $message),
            false,
            false,
            crate::CargoColor::Yellow,
            $stream
        );
    };
}

/// Print an error like cargo message.
///
/// The status is not justified, not bold and light red with the status text 'error'.
///
/// ## Example
/// ```ignore
/// #[macro_use] extern crate carlog;
///
/// use carlog::prelude::*;
///
/// carlog_error!("carlog (v0.1.0) generated an error!");
/// let mut output = Vec::<u8>::new();
/// carlog_error!("carlog (v0.1.0) generated an error!", CarlogStream::Custom(&mut output));
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[macro_export]
macro_rules! carlog_error {
    ($message:expr) => {
        carlog_error!($message, crate::CarlogStream::default());
    };
    ($message:expr, $stream:expr) => {
        carlog!(
            "error",
            format!(": {}", $message),
            false,
            false,
            crate::CargoColor::Red,
            $stream
        );
    };
}

#[cfg(test)]
mod test {
    use crate::CarlogStream;

    #[test]
    fn test_carlog_info() {
        let mut output = Vec::<u8>::new();
        carlog_info!(
            "Compiling",
            "carlog v0.1.0",
            CarlogStream::Custom(&mut output)
        );
        let output = String::from_utf8(output);
        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            "   \u{1b}[1;36mCompiling\u{1b}[0m carlog v0.1.0\n"
        );
    }

    #[test]
    fn test_carlog_ok() {
        let mut output = Vec::<u8>::new();
        carlog_ok!(
            "Compiled",
            "carlog v0.1.0",
            CarlogStream::Custom(&mut output)
        );
        let output = String::from_utf8(output);
        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            "    \u{1b}[1;32mCompiled\u{1b}[0m carlog v0.1.0\n"
        );
    }

    #[test]
    fn test_carlog_warning() {
        let mut output = Vec::<u8>::new();
        carlog_warning!(
            "carlog (v0.1.0) generated a warning!",
            CarlogStream::Custom(&mut output)
        );
        let output = String::from_utf8(output);
        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            "\u{1b}[93mwarning\u{1b}[0m: carlog (v0.1.0) generated a warning!\n"
        );
    }

    #[test]
    fn test_carlog_error() {
        let mut output = Vec::<u8>::new();
        carlog_error!(
            "carlog (v0.1.0) generated an error!",
            CarlogStream::Custom(&mut output)
        );
        let output = String::from_utf8(output);
        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            "\u{1b}[91merror\u{1b}[0m: carlog (v0.1.0) generated an error!\n"
        );
    }
}
