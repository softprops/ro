#![feature(plugin)]
#[plugin] #[no_link]
extern crate json_macros;


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
   let params = Params::empty().username("foo");
   //println!("{}", json!({"username": params.username}).pretty().to_string())
}
