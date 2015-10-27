extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use hyper::Client;
use hyper::method::Method;
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, json};
use std::io::Read;
use url::form_urlencoded;

pub mod errors;
pub use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct YoUser<'a> {
    yo: &'a Yo<'a>,
    username: &'a str
}

impl <'a>YoUser <'a>{
    pub fn new(yo: &'a Yo<'a>, username: &'a str) -> YoUser<'a> {
        YoUser {
            yo: yo,
            username: username
        }
    }

    pub fn yo(&self) -> Result<()> {
        self.yo.post(
            "/yo/",
            &mut vec![
                ("username", self.username)
            ]
        )
    }

    /// yo a link
    pub fn link(&self, l: &'a str) -> Result<()> {
        self.yo.post(
            "/yo/",
            &mut vec![
                ("link", l),
                ("username", self.username)
            ]
        )
    }

    /// yo a location
    pub fn location(&self, lat: f32, lon: f32) -> Result<()> {
        self.yo.post(
            "/yo/",
            &mut vec![
                ("username", self.username),
                ("lat", &lat.to_string()),
                ("lon", &lon.to_string())
            ]
         )
    }
}

pub struct YoAll<'a> {
    yo: &'a Yo<'a>
}

impl <'a>YoAll <'a>{
    pub fn new(yo: &'a Yo<'a>) -> YoAll<'a> {
        YoAll {
            yo: yo
        }
    }

    pub fn link(&self, l: &str) -> Result<()> {
        self.yo.post(
            "/yoall/",
             &mut vec![("link", l)]
        )
    }

}

/// Entrypoint for yo
pub struct Yo<'a> {
    client: &'a Client,
    token: &'a str
}

impl<'a> Yo<'a> {
    pub fn new(client: &'a Client, token: &'a str) -> Yo<'a> {
        Yo {
            client: client,
            token: token
        }
    }

    pub fn yo(&self, who: &'a str) -> YoUser {
        YoUser::new(self, who)
    }

    pub fn yoall(&self) -> YoAll {
        YoAll::new(self)
    }

    pub fn subscribers_count(&self) -> Result<()> {
        self.get("/subscribers_count/", &mut vec![])
    }

    fn get(&self, uri: &'a str, params: &mut Vec<(&'a str, &'a str)>) -> Result<()> {
        params.push(("api_token", self.token));
        self.request(
            Method::Get,
            &format!("{}?{}", uri, form_urlencoded::serialize(params)),
            None
        )
    }

    fn post(&self, uri: &'a str, params: &mut Vec<(&'a str, &'a str)>) -> Result<()> {
        params.push(("api_token", self.token));
        self.request(
            Method::Post,
            uri,
            Some(
                &form_urlencoded::serialize(
                    params
                  ).as_bytes()
              )
        )
    }

    fn request<D: Decodable>(
        &self,
        method: Method,
        uri: &str,
        body: Option<&'a [u8]>
     ) -> Result<D> {
        let url = &format!("http://api.justyo.co{}", uri);
        let builder = self.client.request(method, url);
        let mut res = try!(
            match body {
                Some(ref bod) => {
                    builder.header(ContentType::form_url_encoded())
                        .body(*bod)
                        .send()
                }, _ => builder.send()
            }
        );
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        match res.status {
            StatusCode::BadRequest
            | StatusCode::UnprocessableEntity
            | StatusCode::Unauthorized
            | StatusCode::NotFound
            | StatusCode::Forbidden => Err(
                Error::Fault { code: res.status, body: body }
            ),
            _ => Ok(json::decode::<D>(&body).unwrap())
        }
    }
}
