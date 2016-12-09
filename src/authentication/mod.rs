use rustc_serialize::base64::{Config, Newline, CharacterSet};
use hyper::header::Headers;
use utils::squeeze_path;
use std::ascii::AsciiExt;

pub mod auth13;
use authentication::auth13::Auth13;

pub static BASE64_AUTH: Config = Config {
    char_set: CharacterSet::Standard,
    newline: Newline::LF,
    pad: true,
    line_length: Some(60),
};

#[derive(Clone)]
pub struct Authentication {
    api_version: Option<String>,
    body: Option<String>,
    keypath: String,
    method: String,
    path: String,
    userid: String,
    version: String,
}

impl Authentication {
    pub fn new<P, K, M, U, V>(path: P, key: K, method: M, userid: U, version: V) -> Authentication
        where P: Into<String>,
              K: Into<String>,
              M: Into<String>,
              U: Into<String>,
              V: Into<String>
              {
                  Authentication {
                      api_version: None,
                      body: None,
                      keypath: key.into(),
                      method: method.into().to_ascii_uppercase(),
                      path: squeeze_path(path.into()),
                      userid: userid.into(),
                      version: version.into(),
                  }

              }

    pub fn api_version<S>(mut self, api_version: S) -> Authentication
        where S: Into<String>
        {
            self.api_version = Some(api_version.into());
            self
        }

    pub fn body<S>(mut self, body: S) -> Authentication
        where S: Into<String>
        {
            self.body = Some(body.into());
            self
        }

    pub fn headers(self) -> Headers {
        Auth13::new(&self.path,
                    &self.keypath,
                    &self.method,
                    &self.userid,
                    &self.api_version.unwrap(),
                    self.body)
            .headers().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const PATH: &'static str = "/organizations/clownco";
    const BODY: &'static str = "Spec Body";
    const USER: &'static str = "spec-user";

    const PRIVATE_KEY: &'static str = "fixtures/spec-user.pem";

    #[test]
    fn test_auth_return() {
        let auth = Authentication::new(PATH, PRIVATE_KEY, "GET", USER, "1.3");
        let headers = auth.body(BODY).api_version("1").headers();
        assert!(headers.get_raw("x-ops-authorization-1").is_some())
    }
}
