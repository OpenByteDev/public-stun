use const_format::concatcp;
use thiserror::Error;

use crate::{Capabilities, Capability, InvalidFormat, Server};

const BASE_URL: &str =
    "https://raw.githubusercontent.com/pradt2/always-online-stun/refs/heads/master/";

const VALID_HOSTS_URL: &str = concatcp!(BASE_URL, "valid_hosts.txt");
const VALID_HOSTS_TCP_URL: &str = concatcp!(BASE_URL, "valid_hosts_tcp.txt");
const VALID_NAT_TESTING_HOSTS_URL: &str = concatcp!(BASE_URL, "valid_nat_testing_hosts.txt");
const VALID_NAT_TESTING_HOSTS_TCP_URL: &str =
    concatcp!(BASE_URL, "valid_nat_testing_hosts_tcp.txt");

// const VALID_IPV4S_URL: &str = concatcp!(BASE_URL, "valid_ipv4s.txt");
// const VALID_IPV4S_TCP_URL: &str = concatcp!(BASE_URL, "valid_ipv4s_tcp.txt");
// const VALID_NAT_TESTING_IPV4S_URL: &str = concatcp!(BASE_URL, "valid_nat_testing_ipv4s.txt");
// const VALID_NAT_TESTING_IPV4S_TCP_URL: &str = concatcp!(BASE_URL, "valid_nat_testing_ipv4s_tcp.txt");

// const VALID_IPV6S_URL: &str = concatcp!(BASE_URL, "valid_ipv6s.txt");
// const VALID_IPV6S_TCP_URL: &str = concatcp!(BASE_URL, "valid_ipv6s_tcp.txt");
// const VALID_NAT_TESTING_IPV6S_URL: &str = concatcp!(BASE_URL, "valid_nat_testing_ipv6s.txt");
// const VALID_NAT_TESTING_IPV6S_TCP_URL: &str = concatcp!(BASE_URL, "valid_nat_testing_ipv6s_tcp.txt");

async fn fetch_text_from_url(url: impl reqwest::IntoUrl) -> reqwest::Result<String> {
    reqwest::get(url).await?.text().await
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load server list")]
    Load { url: String, source: reqwest::Error },
    #[error("failed to parse server list")]
    Parse { text: String, source: InvalidFormat },
}

/// Load all known public stun servers regardless of capabilities.
/// 
/// # Errors
/// Fails when either the public server list cannot be fetched or is of invalid format.
pub async fn list_servers() -> Result<Vec<Server>, Error> {
    list_servers_with_caps(Capabilities::empty())
}

/// Load all known public stun servers that support the given capabilities.
/// 
/// # Errors
/// Fails when either the public server list cannot be fetched or is of invalid format.
pub async fn list_servers_with_capabilities(
    caps: impl Into<Capabilities>,
) -> Result<Vec<Server>, Error> {
    let caps = caps.into();
    let url = match (
        caps.contains(Capability::NatTesting),
        caps.contains(Capability::TcpSupport),
    ) {
        (false, false) => VALID_HOSTS_URL,
        (false, true) => VALID_HOSTS_TCP_URL,
        (true, false) => VALID_NAT_TESTING_HOSTS_URL,
        (true, true) => VALID_NAT_TESTING_HOSTS_TCP_URL,
    };
    let text = fetch_text_from_url(url)
        .await
        .map_err(|source| Error::Load {
            url: url.to_string(),
            source,
        })?;
    crate::parse_raw_server_list(&text).map_err(|source| Error::Parse { text, source })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn all() {
        let servers = list_servers().await.unwrap();
        assert!(!servers.is_empty());
    }

    #[tokio::test]
    pub async fn any_for_nat_testing() {
        let servers = list_servers_with_capabilities(Capability::NatTesting)
            .await
            .unwrap();
        assert!(!servers.is_empty());
    }
}
