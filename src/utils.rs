use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct URL<'a> {
  pub hostname: Cow<'a, str>,
  pub is_https: bool,
  pub path: Cow<'a, str>,
}

impl Display for URL<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}{}", &self.hostname, if self.is_https { 443 } else { 80 }, &self.path)
  }
}