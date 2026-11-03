#![cfg(all(feature = "toml", feature = "json"))]
use figment2::{Figment, providers::{Format, Json, Toml}};

fn source(figment: &Figment, key: &str) -> String {
    figment.find_metadata(key).expect("metadata").name.to_string()
}

#[test]
fn replaced_array_keeps_the_metadata_of_its_provider() {
    let defaults = || Figment::from(Json::string(r#"{ "libs": ["lib"], "dict": {} }"#));

    // The array comes from the TOML string, so its metadata must name it too.
    let figment = defaults().merge(Toml::string("libs = []"));
    assert_eq!(figment.extract_inner::<Vec<String>>("libs").unwrap(), Vec::<String>::new());
    assert_eq!(source(&figment, "libs"), "TOML source string");

    let figment = defaults().admerge(Toml::string(r#"libs = ["more"]"#));
    assert_eq!(source(&figment, "libs"), "TOML source string");

    let figment = defaults().zipmerge(Toml::string(r#"libs = ["other"]"#));
    assert_eq!(source(&figment, "libs"), "TOML source string");

    // Joins keep the existing array, and with it its metadata.
    let figment = defaults().join(Toml::string("libs = []"));
    assert_eq!(figment.extract_inner::<Vec<String>>("libs").unwrap(), ["lib"]);
    assert_eq!(source(&figment, "libs"), "JSON source string");

    let figment = defaults().adjoin(Toml::string(r#"libs = ["more"]"#));
    assert_eq!(source(&figment, "libs"), "JSON source string");

    let figment = defaults().zipjoin(Toml::string(r#"libs = ["other"]"#));
    assert_eq!(source(&figment, "libs"), "JSON source string");

    // Dicts already behave this way.
    let figment = defaults().merge(Toml::string("[dict]"));
    assert_eq!(source(&figment, "dict"), "TOML source string");
}
