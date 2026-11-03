//! `world` depends on this crate, so every normal dependency here is linked into the tests of the
//! engine crates. `serde_json` must stay a dev-dependency: its `PartialEq<Value>` impls for the
//! integer types break type inference in `assert_eq!(x, vec![vec![]])` there (see src/json.rs).

#[test]
fn serde_json_is_not_a_normal_dependency() {
    let toml = include_str!("../Cargo.toml");
    let mut section = "";
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            section = line;
            continue;
        }
        if section == "[dependencies]" {
            assert!(
                !line.starts_with("serde_json"),
                "serde_json is back in [dependencies]: {line}"
            );
        }
    }
}
