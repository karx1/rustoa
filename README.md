# RusTOA


[![crates.io](https://img.shields.io/crates/v/rustoa.svg)](https://crates.io/crates/rustoa)
[![Documentation](https://docs.rs/rustoa/badge.svg)](https://docs.rs/rustoa)
[![MIT licensed](https://img.shields.io/crates/l/rustoa.svg)](./LICENSE.md)
[![Build Status](https://ci.karx.xyz/buildStatus/icon?job=rustoa)](https://ci.karx.xyz/job/rustoa/)

Rust bindings to The Orange Alliance API

A Crate to access The Orange Alliance API. This crate makes it easy to access the official First Tech Challenge API and use it in your Rust projects.

Install by adding
```toml
[dependecies]
rustoa = "0.1.7"
```

to your `Cargo.toml` file.

To use the development version, add
```toml
[dependencies]
rustoa = { git = "https://github.com/karx1/rustoa" }
```

to your Cargo.toml file. This version might be unstable.

Here's a simple example:

```rust
use rustoa::{Client, Season};

let client = Client::new("api_key");
let team = client.team(16405);
println!("{}", team.season_wins(Season::SkyStone));
```
