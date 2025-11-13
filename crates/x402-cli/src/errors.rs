use colored::Colorize;
use std::fmt;

/// Exit codes following POSIX conventions
/// Reserved for future use when explicit success exit codes are needed
#[allow(dead_code)]
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERAL: i32 = 1;
pub const EXIT_CONFIG: i32 = 2;
pub const EXIT_NETWORK: i32 = 3;

/// CLI error types with user-friendly formatting
#[derive(Debug)]
pub enum CliError {
    /// Configuration-related errors (exit code 2)
    /// Reserved for Epic 2: Config validation in server startup
    #[allow(dead_code)]
    Config {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    /// Network-related errors (exit code 3)
    /// Reserved for Epic 2: Solana RPC connection errors
    #[allow(dead_code)]
    Network {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    /// Validation errors (exit code 1)
    /// Reserved for Epic 2: Payment validation errors
    #[allow(dead_code)]
    Validation {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    /// I/O errors (exit code 1)
    Io {
        message: String,
        source: std::io::Error,
    },
    /// Other errors (exit code 1)
    Other { message: String },
}

impl CliError {
    /// Get the appropriate exit code for this error type
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Config { .. } => EXIT_CONFIG,
            CliError::Network { .. } => EXIT_NETWORK,
            _ => EXIT_GENERAL,
        }
    }

    /// Get documentation link if available
    pub fn docs_link(&self) -> Option<String> {
        match self {
            CliError::Config { code, .. }
            | CliError::Network { code, .. }
            | CliError::Validation { code, .. } => {
                Some(format!("https://docs.x402-dev.com/errors/{}", code))
            }
            _ => None,
        }
    }

    /// Helper: Create a config error
    /// Reserved for Epic 2: Server startup config validation
    #[allow(dead_code)]
    pub fn config(message: impl Into<String>, code: &'static str) -> Self {
        CliError::Config {
            message: message.into(),
            suggestion: None,
            code,
        }
    }

    /// Helper: Create a config error with suggestion
    /// Reserved for Epic 2: Server startup config validation with hints
    #[allow(dead_code)]
    pub fn config_with_suggestion(
        message: impl Into<String>,
        suggestion: impl Into<String>,
        code: &'static str,
    ) -> Self {
        CliError::Config {
            message: message.into(),
            suggestion: Some(suggestion.into()),
            code,
        }
    }

    /// Helper: Create a network error
    /// Reserved for Epic 2: Solana RPC connection error handling
    #[allow(dead_code)]
    pub fn network(message: impl Into<String>, code: &'static str) -> Self {
        CliError::Network {
            message: message.into(),
            suggestion: None,
            code,
        }
    }

    /// Helper: Create a network error with suggestion
    /// Reserved for Epic 2: Solana RPC connection error handling with hints
    #[allow(dead_code)]
    pub fn network_with_suggestion(
        message: impl Into<String>,
        suggestion: impl Into<String>,
        code: &'static str,
    ) -> Self {
        CliError::Network {
            message: message.into(),
            suggestion: Some(suggestion.into()),
            code,
        }
    }

    /// Helper: Create a validation error
    /// Reserved for Epic 2: Payment validation error handling
    #[allow(dead_code)]
    pub fn validation(message: impl Into<String>, code: &'static str) -> Self {
        CliError::Validation {
            message: message.into(),
            suggestion: None,
            code,
        }
    }

    /// Helper: Create a validation error with suggestion
    /// Reserved for Epic 2: Payment validation error handling with hints
    #[allow(dead_code)]
    pub fn validation_with_suggestion(
        message: impl Into<String>,
        suggestion: impl Into<String>,
        code: &'static str,
    ) -> Self {
        CliError::Validation {
            message: message.into(),
            suggestion: Some(suggestion.into()),
            code,
        }
    }

    /// Helper: Wrap an I/O error
    pub fn io(message: impl Into<String>, source: std::io::Error) -> Self {
        CliError::Io {
            message: message.into(),
            source,
        }
    }

    /// Helper: Create a generic error
    pub fn other(message: impl Into<String>) -> Self {
        CliError::Other {
            message: message.into(),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Config {
                message,
                suggestion,
                ..
            } => {
                writeln!(f, "{} {}", "❌".red(), message.red().bold())?;
                if let Some(hint) = suggestion {
                    write!(f, "{} {}", "💡".yellow(), hint.yellow())?;
                }
            }
            CliError::Network {
                message,
                suggestion,
                ..
            } => {
                writeln!(f, "{} {}", "❌".red(), message.red().bold())?;
                if let Some(hint) = suggestion {
                    write!(f, "{} {}", "💡".yellow(), hint.yellow())?;
                }
            }
            CliError::Validation {
                message,
                suggestion,
                ..
            } => {
                writeln!(f, "{} {}", "❌".red(), message.red().bold())?;
                if let Some(hint) = suggestion {
                    write!(f, "{} {}", "💡".yellow(), hint.yellow())?;
                }
            }
            CliError::Io { message, source } => {
                write!(f, "{} {}: {}", "❌".red(), message.red().bold(), source)?;
            }
            CliError::Other { message } => {
                write!(f, "{} {}", "❌".red(), message.red().bold())?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Convert anyhow::Error to CliError
/// This allows backward compatibility with commands that return anyhow::Result
pub fn convert_anyhow_to_cli_error(error: anyhow::Error) -> CliError {
    // Try to downcast to known error types
    if let Some(io_err) = error.downcast_ref::<std::io::Error>() {
        return CliError::io("I/O operation failed", io_err.kind().into());
    }

    // Default to Other variant with error message
    CliError::other(error.to_string())
}

/// Print error with appropriate formatting and verbosity
pub fn print_error(error: &CliError, verbose: bool, debug: bool) {
    // Print formatted error (uses Display trait with colors)
    eprintln!("{}", error);

    // Print docs link if available
    if let Some(link) = error.docs_link() {
        eprintln!(
            "\n{} {}",
            "📖".cyan(),
            format!("Documentation: {}", link).cyan()
        );
    }

    // Print verbose/debug info
    if debug {
        eprintln!("\n{}", "Debug trace:".dimmed());
        eprintln!("{:?}", error);

        // Print source error chain if available
        if let Some(source) = std::error::Error::source(error) {
            eprintln!("\n{}", "Caused by:".dimmed());
            eprintln!("  {}", source);
        }
    } else if verbose {
        eprintln!("\n{}", "Additional context:".dimmed());

        // Show error type and exit code in verbose mode
        let error_type = match error {
            CliError::Config { .. } => "Configuration Error",
            CliError::Network { .. } => "Network Error",
            CliError::Validation { .. } => "Validation Error",
            CliError::Io { .. } => "I/O Error",
            CliError::Other { .. } => "General Error",
        };

        eprintln!("  Error type: {}", error_type.dimmed());
        eprintln!("  Exit code: {}", error.exit_code().to_string().dimmed());
    }
}
