[![crates-io-badge]][crates-io-link]
![Downloads](https://img.shields.io/crates/d/spectacles-rest.svg?style=for-the-badge)
[![docs-badge]][docs-link]


# Spectacles REST Client
This crate provides a rich interface for making REST requests to the Discord API in your applications.

## Features
* Full coverage of the Discord API.*
* Asynchronous, non-blocking HTTP requests using the [reqwest](https://github.com/seanmonstar/reqwest) library.
* Internal rate-limiting, with support for external HTTP proxies.

\* A handful of endpoints which pertain to Discord OAuth may be omitted.

[crates-io-link]: https://crates.io/crates/spectacles-rest
[crates-io-badge]: https://img.shields.io/crates/v/spectacles-rest.svg?style=for-the-badge
[docs-link]: https://docs.rs/spectacles-rest
[docs-badge]: https://img.shields.io/badge/Documentation-docs.rs-red.svg?style=for-the-badge