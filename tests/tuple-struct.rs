#![cfg(feature = "toml")]
use figment2::Figment;
use figment2::providers::{Format, Toml};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Foo(pub isize);

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    foo: Foo
}

#[test]
fn one_value() {
    let config: Config = Figment::from(Toml::string("foo = 42")).extract().unwrap();
    assert_eq!(config, Config {
        foo: Foo(42)
    })
}
