# serde
No workspace based package -> package dependencies

## [Cargo.toml](https://github.com/serde-rs/serde/blob/master/Cargo.toml)
```toml
[workspace]
members = [
    "serde",
    "serde_derive",
    "serde_derive_internals",
    "test_suite",
]

[patch.crates-io]
serde = { path = "serde" }

[workspace.dependencies]
proc-macro2 = { version = "1.0.74", default-features = false }
quote = { version = "1.0.35", default-features = false }
syn = { version = "2.0.81", default-features = false }
```

## [serde/Cargo.toml](https://github.com/serde-rs/serde/blob/master/serde/Cargo.toml)
```toml
[package]
name = "serde"
version = "1.0.216"

[dependencies]
serde_derive = { version = "1", optional = true, path = "../serde_derive" }
```

## [serde_derive/Cargo.toml](https://github.com/serde-rs/serde/blob/master/serde_derive/Cargo.toml)
```toml
[package]
name = "serde_derive"
version = "1.0.216"

[dev-dependencies]
serde = { version = "1", path = "../serde" }
```

## [serde_derive_internals/Cargo.toml](https://github.com/serde-rs/serde/blob/master/serde_derive_internals/Cargo.toml)
```toml
[package]
name = "serde_derive_internals"
version = "0.29.1"
```

It looks like serde_derive_internals symlinks in serde_derive/src/internals, then builds and repackages them?

-----------------------------------------------------------------------------------------------------------------------

# tokio
No workspace dependencies

## [Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/Cargo.toml)
```toml
[workspace]
resolver = "2"
members = [
  "tokio",
  "tokio-macros",
  "tokio-test",
  "tokio-stream",
  "tokio-util",

  # Internal
  "benches",
  "examples",
  "stress-test",
  "tests-build",
  "tests-integration",
]

[workspace.metadata.spellcheck]
config = "spellcheck.toml"
```

## [tokio/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio/Cargo.toml)
```toml
[package]
name = "tokio"
# When releasing to crates.io:
# - Remove path dependencies
# - Update doc url
#   - README.md
# - Update CHANGELOG.md.
# - Create "v1.x.y" git tag.
version = "1.42.0"

[dependencies]
tokio-macros = { version = "~2.4.0", path = "../tokio-macros", optional = true }

[dev-dependencies]
tokio-test = { version = "0.4.0", path = "../tokio-test" }
tokio-stream = { version = "0.1", path = "../tokio-stream" }
```

## [tokio-macros/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio-macros/Cargo.toml)
```toml
[package]
name = "tokio-macros"
version = "2.4.0"

[dev-dependencies]
tokio = { version = "1.0.0", path = "../tokio", features = ["full"] }
```

## [tokio-stream/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio-stream/Cargo.toml)
```toml
[package]
name = "tokio-stream"
version = "0.1.17"

[dependencies]
tokio = { version = "1.15.0", path = "../tokio", features = ["sync"] }
tokio-util = { version = "0.7.0", path = "../tokio-util", optional = true }

[dev-dependencies]
tokio = { version = "1.2.0", path = "../tokio", features = ["full", "test-util"] }
tokio-test = { version = "0.4", path = "../tokio-test" }
```


## [tokio-test/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio-test/Cargo.toml)
```toml
[package]
name = "tokio-test"
version = "0.4.4"

[dependencies]
tokio = { version = "1.2.0", path = "../tokio", features = ["rt", "sync", "time", "test-util"] }
tokio-stream = { version = "0.1.1", path = "../tokio-stream" }

[dev-dependencies]
tokio = { version = "1.2.0", path = "../tokio", features = ["full"] }
```

## [tokio-util/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio-util/Cargo.toml)
```toml
[package]
name = "tokio-util"
version = "0.7.13"

[dependencies]
tokio = { version = "1.28.0", path = "../tokio", features = ["sync"] }

[dev-dependencies]
tokio = { version = "1.0.0", path = "../tokio", features = ["full"] }
tokio-test = { version = "0.4.0", path = "../tokio-test" }
tokio-stream = { version = "0.1", path = "../tokio-stream" }
```
