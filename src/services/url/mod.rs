//! URL parsing and formatting utilities.

#[cfg(test)]
mod tests;

use std::fmt::Display;

use anyhow::{bail, Error, Result};
use derive_builder::Builder;

#[derive(Debug, Builder, Clone, Eq, PartialEq)]

/// A URL object that can be parsed and formatted.
///
/// * `protocol`: The protocol of the URL. Defaults to "https".
/// * `host`: The host of the URL.
/// * `port`: The port of the URL.
/// * `path`: The path of the URL. Representes the path of the URL as a vector of strings.
/// * `query`: The query of the URL. Represents the query of the URL as a vector of 2-tuples. The
///    first element of the tuple is the key and the second element is the value.
/// * `fragment`: The fragment of the URL.
pub struct Url {
    #[builder(default = r#""https".to_owned()"#)]
    pub protocol: String,
    #[builder(setter(into))]
    pub host: String,
    #[builder(default)]
    pub port: Option<u16>,
    #[builder(default = "vec![]", setter(each(name = "push_path", into)))]
    pub path: Vec<String>,
    #[builder(default = "vec![]", setter(each(name = "push_query", into)))]
    pub query: Vec<(String, String)>,
    #[builder(setter(into), default)]
    pub fragment: Option<String>,
}

impl UrlBuilder {
    /// Parse a string into a URL object.
    ///
    /// * `url`: A string slice that holds the URL.
    pub fn parse(url: &str) -> Result<Self> {
        // Split the URL string into components
        let parts: Vec<&str> = url.splitn(3, "://").collect();

        // Ensure the URL has at least a host
        if parts.len() < 2 {
            bail!("missing host")
        };

        // Extract protocol, host, and the rest of the URL
        let protocol = parts[0].to_owned();
        let rest = parts[1];

        // Extract fragment if present
        let (rest, fragment) = if let Some(fragment_index) = rest.find('#') {
            let (rest_without_fragment, fragment) = rest.split_at(fragment_index);
            (rest_without_fragment, Some(fragment[1..].to_owned()))
        } else {
            (rest, None)
        };

        // Split the remaining part to get host, path, and query
        let mut parts = rest.splitn(2, '/');
        let host = parts.next().unwrap_or("").to_owned();
        let path_query = parts.next().unwrap_or("");

        // Extract port if present
        let (host, port) = if let Some(port_index) = host.find(':') {
            let (host_without_port, port) = host.split_at(port_index);
            let port = port.trim_start_matches(':').parse::<u16>()?;
            (host_without_port.to_owned(), Some(port))
        } else {
            (host, None)
        };

        // Split path_query into path and query
        let mut parts = path_query.splitn(2, '?');
        let path = parts.next().unwrap_or("").split('/').map(String::from).collect();
        let query = parts
            .next()
            .unwrap_or("")
            .split('&')
            .filter_map(|pair| {
                let mut pair_iter = pair.splitn(2, '=');
                let key = pair_iter.next();
                let value = pair_iter.next();
                match (key, value) {
                    (Some(key), Some(value)) => Some((key.to_owned(), value.to_owned())),
                    _ => None,
                }
            })
            .collect();

        Ok(UrlBuilder::default()
            .protocol(protocol)
            .host(host)
            .port(port)
            .path(path)
            .query(query)
            .fragment(fragment)
            .clone())
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = (*self).clone().into();
        write!(f, "{s}")
    }
}

impl<'a> TryFrom<&'a str> for Url {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Split the URL string into components
        let parts: Vec<&str> = value.splitn(3, "://").collect();

        // Ensure the URL has at least a host
        if parts.len() < 2 {
            bail!("missing host")
        };

        // Extract protocol, host, and the rest of the URL
        let protocol = parts[0].to_owned();
        let rest = parts[1];

        // Extract fragment if present
        let (rest, fragment) = if let Some(fragment_index) = rest.find('#') {
            let (rest_without_fragment, fragment) = rest.split_at(fragment_index);
            (rest_without_fragment, Some(fragment[1..].to_owned()))
        } else {
            (rest, None)
        };

        // Split the remaining part to get host, path, and query
        let mut parts = rest.splitn(2, '/');
        let host = parts.next().unwrap_or("").to_owned();
        let path_query = parts.next().unwrap_or("");

        // Extract port if present
        let (host, port) = if let Some(port_index) = host.find(':') {
            let (host_without_port, port) = host.split_at(port_index);
            let port = port.trim_start_matches(':').parse::<u16>()?;
            (host_without_port.to_owned(), Some(port))
        } else {
            (host, None)
        };

        // Split path_query into path and query
        let mut parts = path_query.splitn(2, '?');
        let path = parts.next().unwrap_or("").split('/').map(String::from).collect();
        let query = parts
            .next()
            .unwrap_or("")
            .split('&')
            .map(|pair| {
                let mut pair_iter = pair.splitn(2, '=');
                let key = pair_iter.next().unwrap_or("").to_owned();
                let value = pair_iter.next().unwrap_or("").to_owned();
                (key, value)
            })
            .collect();

        // Construct and return the Url object
        Ok(UrlBuilder::default()
            .protocol(protocol)
            .host(host)
            .port(port)
            .path(path)
            .query(query)
            .fragment(fragment)
            .build()?)
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        // Calculate the total capacity needed for the string
        let capacity = value.protocol.len()
            + 3
            + value.host.len()
            + value.port.map_or(0, |p| p.to_string().len())
            + value.path.iter().map(|p| p.len() + 1).sum::<usize>()
            + value.query.iter().map(|(k, v)| k.len() + v.len() + 1).sum::<usize>()
            + value.fragment.as_ref().map_or(0, |f| f.len() + 1);

        // Preallocate space for the string
        let mut result = String::with_capacity(capacity);

        // Protocol
        result.push_str(&value.protocol);
        result.push_str("://");

        // Host
        result.push_str(&value.host);

        // Port
        if let Some(port) = value.port {
            result.push(':');
            result.push_str(&port.to_string());
        }

        // Path
        if !value.path.is_empty() {
            result.push('/');
            result.push_str(&value.path.join("/"));
        }

        // Query
        if !value.query.is_empty() {
            result.push('?');
            let query_str: Vec<String> =
                value.query.iter().map(|(key, value)| format!("{}={}", key, value)).collect();
            result.push_str(&query_str.join("&"));
        }

        // Fragment
        if let Some(fragment) = value.fragment {
            result.push('#');
            result.push_str(&fragment);
        }

        result
    }
}
