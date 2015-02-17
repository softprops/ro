extern crate hyper;
extern crate mime;
extern crate url;

use hyper::{ Client, Url };
use hyper::header::{ ContentLength, ContentType, UserAgent };
use url::form_urlencoded;
use mime::Mime;
use mime::TopLevel::{ Application, Text };
use mime::SubLevel::{ FormData, WwwFormUrlEncoded };

type Query<'a> = Vec<(&'a str, &'a str)>;

struct Params {
  username: Option<String>,
  loc: Option<(f64,f64)>,
  link: Option<String>
}

impl Params {
  fn empty() -> Params {
    Params { username: None, loc: None, link: None }
  }
  fn loc(mut self, ll: (f64,f64)) -> Params {
    self.loc = Some(ll);
    self.link = None;
    self
  }
  fn link(mut self, ln: &str) -> Params {
    self.link = Some(ln.to_string());
    self.loc = None;
    self
  }
  fn username(mut self, user: &str) -> Params {
    self.username = Some(user.to_string());
    self
  }
}

fn main() {
   let params = vec![("api_token", "{TOKEN}"), ("username", "{USERNAME}")];
   let body = form_urlencoded::serialize(params.into_iter());
   let uri = Url::parse("http://api.justyo.co/yo/").ok().expect("invalid url");
   let mut client = Client::new();
   let res = client.post(uri)
       .header(ContentType(Mime(Application, WwwFormUrlEncoded, vec![])))
       .header(UserAgent("ro/0.1.0".to_string()))
       .body(&*body)
       .send();
    match res {
        Ok(res) => println!("Response: {} {} {}", body.len(), body, res.status),
        Err(e) => println!("Err: {:?}", e)
    }
}