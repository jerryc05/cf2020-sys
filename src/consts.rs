use native_tls::TlsConnector;
use lazy_static::lazy_static;

macro_rules! arr {
  ($vis:vis $id: ident $name: ident: [$ty: ty] = $value: expr) => {
    $vis $id $name: [$ty; $value.len()] = $value;
  }
}

arr!(pub const ARG_PROFILE: [&str] = ["n", "profile", "Number of requests."]);
arr!(pub const ARG_URL:     [&str] = ["url", "url", "The URL to test. Must be one of [http(s)://*.example.com/*] or [*.example.com/*]"]);
arr!(pub const ARG_VERBOSE: [&str] = ["verbose", "verbose", "Enable verbose output."]);

pub const URL_MY_SITE: &str = "https://cf2020.jerryc05.workers.dev";

lazy_static! {
  pub static ref CONNECTOR: TlsConnector = TlsConnector::new().unwrap();
}