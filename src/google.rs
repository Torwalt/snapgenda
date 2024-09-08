use std::{
    self,
    fmt::{self},
    io::{self, Write},
};

use chrono::{NaiveDate, NaiveDateTime};
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, ErrorResponse, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, CONTENT_TYPE},
};

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug)]
pub enum Error {
    Unauthorized(String),
    InputError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unauthorized(e) => {
                write!(f, "Unauthorized error: {}", e)
            }
            Error::InputError(e) => {
                write!(f, "Input error: {}", e)
            }
        }
    }
}

impl<RE, TE> From<oauth2::RequestTokenError<RE, TE>> for Error
where
    RE: std::error::Error + 'static,
    TE: ErrorResponse + 'static,
{
    fn from(error: oauth2::RequestTokenError<RE, TE>) -> Self {
        Error::Unauthorized(error.to_string())
    }
}

impl From<oauth2::url::ParseError> for Error {
    fn from(error: oauth2::url::ParseError) -> Self {
        Error::Unauthorized(error.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Unauthorized(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::InputError(error.to_string())
    }
}

impl std::error::Error for Error {}

pub struct Calendar {
    pub events: Vec<Event>,
}

pub struct Event {
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub summary: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
}

pub fn fetch_calendar(calendar_args: CalendarArgs) -> Result<Calendar, Error> {
    let auth_secret = do_auth(calendar_args.client_args)?;
    let url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        calendar_args.calendar_id
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .query(&[
            (
                "timeMin",
                // 2024-09-08T00:00:00+02:00
                format!(
                    "{}+00:00",
                    calendar_args
                        .from
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .format("%Y-%m-%dT%H:%M:%S")
                ),
            ),
            (
                "timeMax",
                format!(
                    "{}+00:00",
                    calendar_args
                        .to
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .format("%Y-%m-%dT%H:%M:%S")
                ),
            ),
        ])
        .header(AUTHORIZATION, format!("Bearer {}", auth_secret))
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    if !response.status().is_success() {
        return Err(Error::Unauthorized("".to_string()));
    }

    // TODO: Parse json.
    let events: serde_json::Value = response.json()?;

    Ok(Calendar { events: todo!() })
}

pub struct CalendarArgs {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub calendar_id: String,
    pub client_args: ClientArgs,
}

pub struct ClientArgs {
    pub id: String,
    pub secret: String,
}

fn do_auth(client_args: ClientArgs) -> Result<String, Error> {
    let client_id = ClientId::new(client_args.id);
    let client_secret = ClientSecret::new(client_args.secret);
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar.readonly".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open this URL in your browser:\n{}\n", auth_url);

    print!("Enter the authorization code: ");
    io::stdout().flush()?;
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim().to_string();

    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)?;

    Ok(token_result.access_token().secret().to_string())
}
