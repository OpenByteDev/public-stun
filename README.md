# public-stun

[![CI](https://github.com/OpenByteDev/public-stun/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/public-stun/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/public-stun.svg)](https://crates.io/crates/public-stun)
[![Documentation](https://docs.rs/public-stun/badge.svg)](https://docs.rs/public-stun)
[![dependency status](https://deps.rs/repo/github/openbytedev/public-stun/status.svg)](https://deps.rs/repo/github/openbytedev/public-stun)
[![MIT](https://img.shields.io/crates/l/public-stun.svg)](https://github.com/OpenByteDev/public-stun/blob/master/LICENSE)

A list of public STUN servers in crate form.

Based on the public STUN server list maintained by [pradt2](https://github.com/pradt2/always-online-stun).

## Usage
The crate supports to ways to get the list of STUN servers:

Packaged servers:
```rust
// The list is packaged with the crate and may be outdated.
let servers = public_stun::packaged::list_servers();
```

Fetched servers:
```rust
// The list is fetched from the source repository and is always up to date.
let servers = public_stun::fetched::list_servers().await.unwrap();
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/public-stun/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
