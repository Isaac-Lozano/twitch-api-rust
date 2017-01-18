#![feature(custom_attribute)]
#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod model;

use model::user::{UserId, Users};
use model::badges::BadgeSets;

use hyper::client::{Client, Response};
use hyper::net::HttpsConnector;
use hyper::Url;
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper_native_tls::NativeTlsClient;

use std::fmt;
use std::error::Error;

const TWITCH_API_BASE_URL: &'static str = "https://api.twitch.tv/kraken/";
const TWITCH_BADGE_API_BASE_URL: &'static str = "https://badges.twitch.tv/v1/";
const TWITCH_REQUEST_MIME_SUBLEVEL: &'static str = "vnd.twitchtv.v5+json";

header!{(ClientId, "client-id") => [String]}

pub struct TwitchApi
{
    client: Client,
    client_id: String,
}

impl TwitchApi
{
    pub fn new(client_id: &str) -> TwitchApiResult<TwitchApi>
    {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Ok(TwitchApi
        {
            client: Client::with_connector(connector),
            client_id: client_id.into(),
        })
    }

    fn make_twitch_api_request(&self, path: &str, params: Option<&str>, token: Option<&str>) -> TwitchApiResult<Response>
    {
        let mut url = Url::parse(TWITCH_API_BASE_URL).unwrap();
        url = url.join(path).unwrap();
        url.set_query(params);

        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(
                        Mime(TopLevel::Application,
                             SubLevel::Ext(TWITCH_REQUEST_MIME_SUBLEVEL.into()),
                             vec![]))]));
        headers.set(ClientId(self.client_id.clone()));

        if let Some(t) = token
        {
            headers.set(Authorization(String::from(t)));
        }

        Ok(self.client
               .get(url)
               .headers(headers)
               .send()
               .unwrap())
    }

    fn make_badge_api_request(&self, path: &str) -> TwitchApiResult<Response>
    {
        let mut url = Url::parse(TWITCH_BADGE_API_BASE_URL).unwrap();
        url = url.join(path).unwrap();

        Ok(self.client
               .get(url)
               .send()
               .unwrap())
    }

    /* TODO: Grab multiple users from here? */
    pub fn get_user(&self, user: &str) ->TwitchApiResult<Users>
    {
        let path = format!("users");
        let params = format!("login={}", user);
        let resp = self.make_twitch_api_request(&path, Some(&params), None).unwrap();
        Ok(serde_json::from_reader(resp).unwrap())
    }

    pub fn get_badges(&self) -> TwitchApiResult<BadgeSets>
    {
        let path = "badges/global/display";
        let resp = self.make_badge_api_request(path).unwrap();
        Ok(serde_json::from_reader(resp).unwrap())
    }

    pub fn get_subscriber_badges(&self, user: UserId) -> TwitchApiResult<BadgeSets>
    {
        let UserId(id) = user;
        let path = format!("badges/channels/{}/display", id);
        let resp = self.make_badge_api_request(&path).unwrap();
        Ok(serde_json::from_reader(resp).unwrap())
    }
}

#[derive(Debug)]
pub enum TwitchApiError
{
    Unknown,
}


impl fmt::Display for TwitchApiError
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        write!(fmt, "TwitchApiError: {}", self.description())
    }
}

impl Error for TwitchApiError
{
    fn description(&self) -> &str
    {
        match *self
        {
            TwitchApiError::Unknown => "Unknown error",
        }
    }

    fn cause(&self) -> Option<&Error>
    {
        match *self
        {
            TwitchApiError::Unknown => None
        }
    }
}

pub type TwitchApiResult<T> = Result<T, TwitchApiError>;
