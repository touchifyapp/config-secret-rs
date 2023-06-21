# config-secret

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![CI](https://github.com/touchifyapp/config-secret-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/touchifyapp/config-secret-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/d/config-secret.svg)](https://crates.io/crates/config-secret)
[![Docs.rs](https://docs.rs/config-secret/badge.svg)](https://docs.rs/config-secret)

`config-secret` is an additional source for the [config](https://github.com/mehcode/config-rs) crate that follows the Docker/Kubernetes convention.

It allows to inject some parts of your configuration by using a file specified as environment variable. [See examples](#examples).

## Installation

```toml
[dependencies]
config = "0.13"
config-secret = "0.1.0"
```

## Usage

```rust
use config::Config;
use config_secret::EnvironmentSecretFile;

let source = EnvironmentSecretFile::with_prefix("APP").separator("_");
let config = Config::builder().add_source(source).build().unwrap();
let settings = config.try_deserialize::<Settings>().unwrap();

// settings...
```

## Examples

### Definition

Let's introduce our types and our `config` initializer:

```rust
use config::{Config, ConfigError};
use config_secret::EnvironmentSecretFile;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub redis: RedisSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedisSettings {
    pub nodes: Vec<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn get_settings() -> Result<Settings, ConfigError> {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("_")
                .list_separator(",")
                .with_list_parse_key("redis.nodes")
                .try_parsing(true),
        )
        .add_source(
            EnvironmentSecretFile::with_prefix("APP")
                .separator("_")
        )
        .build()?;

    config.try_deserialize::<Settings>()
}
```

### Full configuration

We can add an environment variable to set a secret that configure the whole configuration:

```env
APP_FILE=/run/secrets/my_secret.json
```
```json
{
    "server": {
        "host": "0.0.0.0",
        "port": 5000
    },
    "redis": {
        "nodes": [
            "redis://10.0.0.1:6379",
            "redis://10.0.0.2:6379",
            "redis://10.0.0.3:6379"
        ]
    }
}
```
```rust
let settings = get_settings().unwrap();
assert!(settings.server.host == "0.0.0.0");
```

### Partial configuration

We can add environments variables that set only a sub section of your configuration:

```env
APP_SERVER_HOST=127.0.0.1
APP_SERVER_PORT=5000

APP_REDIS_FILE=/run/secrets/redis.yaml
```
```yaml
nodes:
    - redis://10.0.0.1:6379
    - redis://10.0.0.2:6379
    - redis://10.0.0.3:6379
username: redis
password: superpassword
```
```rust
let settings = get_settings().unwrap();
assert!(settings.server.host == "127.0.0.1");
assert!(settings.redis.username == "redis");
```

### License

[MIT](LICENSE)