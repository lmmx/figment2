# figment2 &thinsp; [![ci.svg]][ci] [![crates.io]][crate] [![docs.rs]][docs]

[crates.io]: https://img.shields.io/crates/v/figment2.svg
[crate]: https://crates.io/crates/figment2
[docs.rs]: https://docs.rs/figment2/badge.svg
[docs]: https://docs.rs/figment2
[ci.svg]: https://github.com/lmmx/figment2/workflows/CI/badge.svg
[ci]: https://github.com/lmmx/figment2/actions

> Remaintained fork of [figment](https://github.com/SergioBenitez/Figment)

Figment2 is a semi-hierarchical configuration library for Rust so con-free, it's
_unreal_.

```rust
use serde::Deserialize;

use figment2::{Figment, providers::{Format, Toml, Json, Env}};

#[derive(Deserialize)]
struct Package {
    name: String,
    authors: Vec<String>,
    publish: Option<bool>,
    // ... and so on ...
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    rustc: Option<String>,
    // ... and so on ...
}

let config: Config = Figment::new()
    .merge(Toml::file("Cargo.toml"))
    .merge(Env::prefixed("CARGO_"))
    .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
    .join(Json::file("Cargo.json"))
    .extract()?;
```

See the [documentation](https://docs.rs/figment2) for a detailed usage guide and
information.

## Usage

Add the following to your `Cargo.toml`, enabling the desired built-in providers:

```toml
[dependencies]
figment2 = { version = "0.10", features = ["toml"] }
```

## License

Figment is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
