# serde
No workspace based package -> package dependencies

## serde/Cargo.toml
```toml
[package]
name = "serde"
version = "1.0.216"

[dependencies]
serde_derive = { version = "1", optional = true, path = "../serde_derive" }
```

## serde_derive/Cargo.toml
```toml
[package]
name = "serde_derive"
version = "1.0.216"

[dev-dependencies]
serde = { version = "1", path = "../serde" }
```

## serde_derive_internals/Cargo.toml
```toml
[package]
name = "serde_derive_internals"
version = "0.29.1"
```

It looks like serde_derive_internals symlinks in serde_derive/src/internals, then builds and repackages them?

-----------------------------------------------------------------------------------------------------------------------

# tokio
No workspace dependencies

## tokio/Cargo.toml
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

## tokio-macros/Cargo.toml
```toml
[package]
name = "tokio-macros"
version = "2.4.0"

[dev-dependencies]
tokio = { version = "1.0.0", path = "../tokio", features = ["full"] }
```

## tokio-stream/Cargo.toml
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


## tokio-test/Cargo.toml
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

## tokio-util/Cargo.toml
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
