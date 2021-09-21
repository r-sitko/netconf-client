[![Rust](https://github.com/rsitko92/netconf-client/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rsitko92/netconf-client/actions/workflows/rust.yml)

<!-- omit in toc -->
# netconf-client

## Table of contents
- [Table of contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
    - [Use library in project](#use-library-in-project)
    - [Run integration tests from netconf-client library](#run-integration-tests-from-netconf-client-library)
  - [Setup](#setup)
  - [Examples](#examples)
  - [Error handling](#error-handling)
- [Running integration tests](#running-integration-tests)
- [API documentation](#api-documentation)

## About The Project
Rust NETCONF RFC6241 client with SSH secure connection support.

## Features
- SSH connection
- Supported NETCONF messages:
  - \<hello>
  - \<get-config>
  - \<edit-config>
  - \<copy-config>
  - \<delete-config>
  - \<lock>
  - \<unlock>
  - \<get>
  - \<close-session>
  - \<kill-session>
  - \<discard-changes>
  - \<commit>
- Supported datastores:
  - \<running>
  - \<candidate>
  - \<startup>

## Getting Started

### Prerequisites
#### Use library in project
- Rust
- Cargo
#### Run integration tests from netconf-client library
- Rust
- Cargo
- Docker

### Setup
Add netconf-client git package to Cargo.toml dependencies section. This library is not available in crates.io.

Basing on latest commit from main branch:
```toml
[dependencies]
netconf-client = { git = "https://github.com/rsitko92/netconf-client.git" }
```

Basing on concrete commit:
```toml
[dependencies]
netconf-client = { git = "https://github.com/rsitko92/netconf-client.git", rev = "9486fdf" }
```

### Examples

All netconf-client API methods returns std::Result type. If client get successful response, std::Result with Ok(T) variant is returned, where T is a apriopriate NETCONF response type defined in [replies](src/models/replies.rs) module.

```rust
let mut client = NetconfClient::new(
    "127.0.0.1",
    830,
    "root",
    "root",
);
client.connect().unwrap();
client.send_hello().unwrap();
let rsp = client.get(None).unwrap();
println!("get response: {:?}", rsp.data);
```

More useful examples can be found in [tests](tests) folder.

### Error handling
When error is encountered (for example: io error, SSH error, NETCONF response error) API methods return std::Result with Err(NetconfClientError) variant. NetconfClientError is an enum defined in [errors](src/errors.rs) module. User should check if methods return Err and react accordingly.

## Running integration tests
```shell
cargo test --test '*'
```


## API documentation
https://rsitko92.github.io/netconf-client/netconf_client/
