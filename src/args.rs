use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Debug)]
pub enum Error {
    InvalidGoogleArgs(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidGoogleArgs(e) => {
                write!(f, "Invalid Google Args: {}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, ValueEnum)]
pub enum Provider {
    GoogleCalendar,
}

#[derive(Parser, Debug)]
#[command(author = "Torwalt", version = "0.1", about = "Snapgenda", long_about = None)]
pub struct Args {
    #[arg(long)]
    pub calendar_email: String,

    #[arg(value_enum, long)]
    pub provider: Provider,

    #[arg(long)]
    pub google_id: Option<String>,

    #[arg(long)]
    pub google_secret: Option<String>,
}

pub struct GoogleArgs {
    pub calendar_email: String,
    pub provider: Provider,
    pub google_id: String,
    pub google_secret: String,
}

impl GoogleArgs {
    pub fn new(args: Args) -> Result<GoogleArgs, Error> {
        Ok(GoogleArgs {
            calendar_email: args.calendar_email,
            provider: Provider::GoogleCalendar,
            google_id: args
                .google_id
                .ok_or(Error::InvalidGoogleArgs("missing google_id".to_string()))?,
            google_secret: args.google_secret.ok_or(Error::InvalidGoogleArgs(
                "missing google_secret".to_string(),
            ))?,
        })
    }
}
