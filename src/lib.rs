//! A list of public STUN servers in crate form.
//!
//! Based on the public STUN server list maintained by [pradt2](https://github.com/pradt2/always-online-stun).
//!
//! ## Usage
//! The crate supports to ways to get the list of STUN servers:
//!
//! Packaged servers:
//! ```rust
//! // The list is packaged with the crate and may be outdated.
//! # #[cfg(feature = "packaged")]
//! let servers = public_stun::packaged::list_servers();
//! ```
//!
//! Fetched servers:
//! ```rust
//! // The list is fetched from the source repository and is always up to date.
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! # #[cfg(feature = "fetched")]
//! let servers = public_stun::fetched::list_servers().await.unwrap();
//! # }
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use core::{
    fmt::{self, Display},
    str::FromStr,
};
use std::io;

use enumflags2::{BitFlags, bitflags};
use thiserror::Error;

/// The required capabilities of a STUN server.
#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability {
    /// The server supports NAT detection and behaviour testing (roughly correspond to RFC5780).
    NatTesting,
    /// The server supports STUN via TCP.
    TcpSupport,
}

/// A set of [`Capability`]s.
pub type Capabilities = BitFlags<Capability>;

/// The hostname and port of a STUN server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Server {
    /// The hostname of the STUN server.
    pub host: String,
    /// The port of the STUN server.
    pub port: u16,
}

impl Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl std::net::ToSocketAddrs for Server {
    type Iter = <(String, u16) as std::net::ToSocketAddrs>::Iter;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        (self.host.as_str(), self.port).to_socket_addrs()
    }
}

/// The error returned when parsing a [`Server`] from a [`str`].
#[derive(Debug, Error)]
pub enum InvalidFormat {
    #[error("missing ':' in source")]
    MissingDelimiter,
    #[error("port number is not a valid u16")]
    InvalidPort,
}

impl FromStr for Server {
    type Err = InvalidFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((host, port)) = s.split_once(':') else {
            return Err(InvalidFormat::MissingDelimiter);
        };
        let host = host.to_string();
        let Ok(port) = port.parse() else {
            return Err(InvalidFormat::InvalidPort);
        };
        Ok(Self { host, port })
    }
}

#[cfg(any(feature = "packaged", feature = "fetched"))]
pub(crate) fn parse_raw_server_list(text: &str) -> Result<Vec<Server>, InvalidFormat> {
    text.lines().map(|line| line.trim().parse()).collect()
}

/// Module containing the packaged version of the server list.
#[cfg(feature = "packaged")]
pub mod packaged;

/// Module containing the fetched (on-demand) version of the server list.
#[cfg(feature = "fetched")]
pub mod fetched;
