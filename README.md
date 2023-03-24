# buildkit-rs

According to the [buildkit repo](https://github.com/moby/buildkit) buildkit is:

> a toolkit for converting source code to build artifacts in an efficient, expressive and repeatable manner

This is a Rust client library for buildkit.

## Crates

- [buildkit-rs](/) - The meta crate for the buildkit-rs project, it contains all the other crates reexported with no other functionality
- [buildkit-rs-llb](/crates/llb) - The low level buildkit client library
- [buildkit-rs-proto](/crates/proto) - The buildkit protobuf definitions
- [buildkit-rs-util](/crates/util) - Utilities for building applications that use buildkit

### Planned crates

- [buildkit-rs-client](/) - A high level client library for buildkit exposing a simmilar API to the Go client
- [buildkit-rs-dockerfile](/) - A library for parsing and converting Dockerfiles to LLB (this is mostly for validation and testing, not for production use)

## QA

### What are the goals of this project?

#### Goals

In order of importance:

- Provide a simple, safe, and fast buildkit client library for Rust
- Provide other utilities for building applications that use buildkit in Rust
- Keeping the API simmilar to the buildkit Go client API
- Keeping the code modular as to allow easy opt-in for features

#### Non-goals (non-exhaustive)

- Provide a buildkit daemon implementation
- Provide a CLI for buildkit
- Provide a production ready Dockerfile llb converter

In short, this project is not trying to replace buildkit, but rather provide a Rust client library for buildkit.

### What about the [rust-buildkit](https://github.com/denzp/rust-buildkit)

It is a similar project, but it is not maintained anymore and it is not compatible with the latest buildkit version. This project is backed by [Cicada](https://cicada.build) and we are using it in our project.

### Why not use the [buildkit Go client](https://github.com/moby/buildkit) directly?

The Go client is a great library, but it is not easy to use in Rust. A native Rust client library is much easier to use and it is much faster for our use case. We are all in on Rust but still want to leverage the buildkit ecosystem.

## License

Apache-2.0 OR MIT

## Contributing

Any contributions are welcome! If you are interested in contributing, please open an issue or a PR as soon as possible so we can discuss it and ensure it fits the goals of the project + we can avoid duplicate work.

We also are welcome to discuss the project in the [Cicada Discord](https://discord.gg/8qZ3Z5Z) in the #buildkit-rs channel.

