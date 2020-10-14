#![feature(in_band_lifetimes)]

use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};

use clap::{App, Arg};

use consts::{ARG_PROFILE, ARG_URL, ARG_VERBOSE, CONNECTOR, URL_MY_SITE};
use utils::URL;

mod consts;
mod utils;


fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .arg(Arg::with_name(ARG_PROFILE[0])
          .short(&ARG_PROFILE[1][0..=1])
          .long(ARG_PROFILE[1])
          .help(ARG_PROFILE[2])
          .takes_value(true))
      .arg(Arg::with_name(ARG_URL[0])
          .short(&ARG_URL[1][0..=1])
          .long(ARG_URL[1])
          .help(ARG_URL[2])
          .takes_value(true))
      .arg(Arg::with_name(ARG_VERBOSE[0])
          .short(&ARG_VERBOSE[1][0..=1])
          .long(ARG_VERBOSE[1])
          .help(ARG_VERBOSE[2])
          .takes_value(false))
      .get_matches();

  let verbose = matches.is_present(ARG_VERBOSE[0]);

  match (matches.value_of(ARG_URL[0]), matches.value_of(ARG_PROFILE[0])) {
    (Some(url), None) => {
      request(&parse_url(url, verbose));
    }
    (url_opt, Some(n)) => {
      let url = url_opt.unwrap_or(URL_MY_SITE);
      profile(&parse_url(url, verbose),
              n.parse::<usize>().expect(&format!("Invalid number of profile runs: [{}]", n)));
    }
    (None, None) => {
      panic!("No arguments provided. Run with \"-h\" to show help message.");
    }
  }
}

fn parse_url(mut url: &str, verbose: bool) -> URL {
  const HTTP: &str = "http";
  const HTTP_PF: &str = "://";
  const HTTPS_PF: &str = "s://";

  let addr;
  let is_https;
  let path;

  if url.starts_with(HTTP) {
    url = &url[HTTP.len()..];

    if url.starts_with(&HTTP_PF[0..=1]) {
      is_https = false;
      url = &url[HTTP_PF.len()..];
    } else {
      debug_assert!(url.starts_with(HTTPS_PF));
      is_https = true;
      url = &url[HTTPS_PF.len()..];
    }
  } else {
    is_https = false;
    url = url;
  }

  let port_idx = url.find('/').unwrap_or(url.len());

  addr = &url[..port_idx];
  path = if port_idx == url.len() { "/" } else { &url[port_idx..] };

  let rv = URL {
    hostname: Cow::from(addr),
    is_https,
    path: Cow::from(path),
  };

  if verbose {
    println!("[VERBOSE] URL [{}] parsed to: [{}]", url, rv);
  }

  rv
}

fn request(addr: &URL) {
  let tcp_stream = {
    let s = format!("{}:{}", &addr.hostname, if addr.is_https { 443 } else { 80 });
    TcpStream::connect(&s).expect(&format!("TcpStream failed to connect to: [{}]", &s))
  };
  let mut tls_stream = {
    let s = &addr.hostname;
    CONNECTOR.connect(s, tcp_stream)
        .expect(&format!("TlsStream failed to connect to: [{}]", s))
  };

  let s = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", &addr.path, &addr.hostname);
  println!("{}",s);
  tls_stream.write_all(s.as_bytes()).unwrap();

  let mut buf = vec![];
  tls_stream.read_to_end(&mut buf).unwrap();
  println!("{}", String::from_utf8_lossy(&buf));
}

fn profile(addr: &URL, n: usize) {}


#[test]
fn test_parse_url() {
  assert_eq!(URL { hostname: "google.com".into(), is_https: true, path: "".into() },
             parse_url("https://google.com".into(), false));
  assert_eq!(URL { hostname: "google.com".into(), is_https: false, path: "".into() },
             parse_url("http://google.com".into(), false));
  assert_eq!(URL { hostname: "google.com".into(), is_https: false, path: "".into() },
             parse_url("google.com".into(), false));

  assert_eq!(URL { hostname: "google.com".into(), is_https: true, path: "/123".into() },
             parse_url("https://google.com/123".into(), false));
  assert_eq!(URL { hostname: "google.com".into(), is_https: false, path: "/123".into() },
             parse_url("http://google.com/123".into(), false));
  assert_eq!(URL { hostname: "google.com".into(), is_https: false, path: "/123".into() },
             parse_url("google.com/123".into(), false));
}

/*
fn dns_lookup(url: &str, verbose: bool) -> SocketAddr {
  match SocketAddr::from_str(url) {
    Ok(ip) => ip,
    _ => {
      lookup_host(url).map(|ips| {
        if verbose {
          println!("[VERBOSE] hostname {} resolved to {}", url, ips[0]);
        }
        SocketAddr::new(ips[0], { if url.starts_with("https") { 443 } else { 80 } })
      }).expect(&format!("Failed to resolve hostname: [{}]", url))
    }
  }
}*/