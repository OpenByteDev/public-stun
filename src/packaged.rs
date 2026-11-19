use crate::{Capabilities, Capability, Server};

const VALID_HOSTS: &str = include_str!("../packaged/valid_hosts.txt");
const VALID_HOSTS_TCP: &str = include_str!("../packaged/valid_hosts_tcp.txt");
const VALID_NAT_TESTING_HOSTS: &str = include_str!("../packaged/valid_nat_testing_hosts.txt");
const VALID_NAT_TESTING_HOSTS_TCP: &str =
    include_str!("../packaged/valid_nat_testing_hosts_tcp.txt");

// const VALID_IPV4S: &str = include_str!("../packaged/valid_ipv4s.txt");
// const VALID_IPV4S_TCP: &str = include_str!("../packaged/valid_ipv4s_tcp.txt");
// const VALID_NAT_TESTING_IPV4S: &str = include_str!("../packaged/valid_nat_testing_ipv4s.txt");
// const VALID_NAT_TESTING_IPV4S_TCP: &str = include_str!("../packaged/valid_nat_testing_ipv4s_tcp.txt");

// const VALID_IPV6S: &str = include_str!("../packaged/valid_ipv6s.txt");
// const VALID_IPV6S_TCP: &str = include_str!("../packaged/valid_ipv6s_tcp.txt");
// const VALID_NAT_TESTING_IPV6S: &str = include_str!("../packaged/valid_nat_testing_ipv6s.txt");
// const VALID_NAT_TESTING_IPV6S_TCP: &str = include_str!("../packaged/valid_nat_testing_ipv6s_tcp.txt");

/// Load all known public stun servers regardless of capabilities.
#[must_use]
pub fn list_servers() -> Vec<Server> {
    list_servers_with_capabilities(Capabilities::empty())
}

/// Load all known public stun servers that support the given capabilities.
pub fn list_servers_with_capabilities(caps: impl Into<Capabilities>) -> Vec<Server> {
    let caps = caps.into();
    let text = match (
        caps.contains(Capability::NatTesting),
        caps.contains(Capability::TcpSupport),
    ) {
        (false, false) => VALID_HOSTS,
        (false, true) => VALID_HOSTS_TCP,
        (true, false) => VALID_NAT_TESTING_HOSTS,
        (true, true) => VALID_NAT_TESTING_HOSTS_TCP,
    };
    crate::parse_raw_server_list(text).expect("packaged server lists are of invalid format")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn all() {
        let servers = list_servers();
        assert_ne!(servers, [] as [Server; 0]);
    }

    #[test]
    pub fn any_for_nat_testing() {
        let servers = list_servers_with_capabilities(Capability::TcpSupport);
        assert_ne!(servers, [] as [Server; 0]);
    }
}
