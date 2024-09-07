use std::{
    self, fmt,
    io::{self, Write},
};

use chrono::{NaiveDate, NaiveDateTime};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, CONTENT_TYPE},
};

const AUTH_URL: String = String::from("https://accounts.google.com/o/oauth2/auth");
const TOKEN_URL: String = String::from("https://oauth2.googleapis.com/token");

#[derive(Debug)]
enum Error {
    Unauthorized(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unauthorized(e) => write!(f, "Unauthorized error: {}", e),
        }
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
        Error::Unauthorized(error.to_string())
    }
}

impl std::error::Error for Error {}

struct Calendar {
    events: Vec<Event>,
}

struct Event {
    created: NaiveDateTime,
    updated: NaiveDateTime,
    summary: String,
    from: NaiveDate,
    to: NaiveDate,
}

struct Request {
    api_key: String,
    // I.e. the email address the calendar is tied to.
    calendar_id: String,
    from: NaiveDateTime,
    to: NaiveDateTime,
}

pub fn fetch_calendar(calendar_args: CalendarArgs) -> Result<Calendar, Error> {
    let auth_secret = auth(calendar_args.client_args)?;
    let url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        calendar_args.calendar_id
    );

    let client = Client::new();
    let response = client
        .get(&url)
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
    calendar_id: String,
    client_args: ClientArgs,
}

pub struct ClientArgs {
    pub id: String,
    pub secret: String,
}

fn auth(client_args: ClientArgs) -> Result<String, Error> {
    let client_id = ClientId::new(client_args.id);
    let client_secret = ClientSecret::new(client_args.secret);
    let auth_url = AuthUrl::new(AUTH_URL)?;
    let token_url = TokenUrl::new(TOKEN_URL)?;

    // Set up the OAuth2 client
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    // Generate a PKCE code verifier and challenge
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar.readonly".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Open the authorization URL in the user's browser
    println!("Open this URL in your browser:\n{}\n", auth_url);

    // Wait for the user to enter the authorization code
    print!("Enter the authorization code: ");
    io::stdout().flush()?;
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim().to_string();
    let req_client = reqwest::Client::new();

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .set_pkce_verifier(pkce_verifier)
        .request(req_client)?;

    Ok(token_result)
}
