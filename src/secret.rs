use std::{env, path::Path};

use config::{ConfigError, File, Map, Source, Value, ValueKind};

#[derive(Clone, Debug, Default)]
pub struct EnvironmentSecretFile {
    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the secret environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    prefix: Option<String>,

    /// Optional character sequence that separates the prefix from the rest of the key
    /// Defaults to `separator` or `_`
    prefix_separator: Option<String>,

    /// Suffix that will limit secrets in the environment to only keys that ends with the defined
    /// prefix.
    ///
    /// A suffix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the secret environment.
    ///
    /// The default value is `FILE`.
    ///
    /// For example, the key `CONFIG_FILE` would parse the file pointed in the variable and collect
    /// the content config into the key `config`.
    suffix: Option<String>,

    /// Optional character sequence that separates the prefix from the rest of the key
    /// Defaults to `separator` or `_`
    suffix_separator: Option<String>,

    /// Optional character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    separator: Option<String>,

    // Preserve the prefix while parsing
    keep_prefix: bool,
}

impl EnvironmentSecretFile {
    pub fn with_prefix(s: &str) -> Self {
        Self {
            prefix: Some(s.into()),
            ..Self::default()
        }
    }

    pub fn prefix(mut self, s: &str) -> Self {
        self.prefix = Some(s.into());
        self
    }

    pub fn prefix_separator(mut self, s: &str) -> Self {
        self.prefix_separator = Some(s.into());
        self
    }

    pub fn suffix(mut self, s: &str) -> Self {
        self.suffix = Some(s.into());
        self
    }

    pub fn suffix_separator(mut self, s: &str) -> Self {
        self.suffix_separator = Some(s.into());
        self
    }

    pub fn separator(mut self, s: &str) -> Self {
        self.separator = Some(s.into());
        self
    }

    pub fn keep_prefix(mut self, keep: bool) -> Self {
        self.keep_prefix = keep;
        self
    }
}

impl Source for EnvironmentSecretFile {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let mut m = Map::new();

        let separator = self.separator.as_deref().unwrap_or("");
        let prefix_separator = match (self.prefix_separator.as_deref(), self.separator.as_deref()) {
            (Some(pre), _) => pre,
            (None, Some(sep)) => sep,
            (None, None) => "_",
        };
        let suffix_separator = match (self.suffix_separator.as_deref(), self.separator.as_deref()) {
            (Some(suf), _) => suf,
            (None, Some(sep)) => sep,
            (None, None) => "_",
        };

        let prefix_pattern = self
            .prefix
            .as_ref()
            .map(|prefix| format!("{}{}", prefix, prefix_separator).to_lowercase());

        let suffix = self.suffix.as_ref().map_or_else(|| "FILE", |s| s.as_str());
        let suffix_pattern = format!("{}{}", suffix_separator, suffix).to_lowercase();

        let full_pattern = if let Some(prefix) = self.prefix.as_ref() {
            if prefix_separator == suffix_separator {
                format!("{}{}{}", prefix, prefix_separator, suffix).to_lowercase()
            } else {
                format!("{}{}", prefix, suffix)
            }
        } else {
            suffix.to_string()
        };

        let mut error: Option<ConfigError> = None;

        env::vars().for_each(|(key, value): (String, String)| {
            // Stop processing on error
            if let Some(_) = error.as_ref() {
                return;
            }

            // Treat empty environment variables as unset
            if value.is_empty() {
                return;
            }

            let mut key = key.to_lowercase();

            if key == full_pattern {
                let path = Path::new(&value);
                let file = File::from(path);
                let map = file.collect();

                match map {
                    Ok(map) => {
                        for (key, value) in map.into_iter() {
                            m.insert(key, value);
                        }
                    }
                    Err(err) => {
                        error = Some(err);
                    }
                }

                return;
            }

            // Check for prefix
            if let Some(ref prefix_pattern) = prefix_pattern {
                if key.starts_with(prefix_pattern) {
                    if !self.keep_prefix {
                        // Remove this prefix from the key
                        key = key[prefix_pattern.len()..].to_string();
                    }
                } else {
                    // Skip this key
                    return;
                }
            }

            // Check for suffix
            if key.ends_with(&suffix_pattern) {
                // Remove this suffix from the key
                let len = key.len() - suffix_pattern.len();
                key = key[..len].to_string();
            } else {
                // Skip this key
                return;
            }

            // If separator is given replace with `.`
            if !separator.is_empty() {
                key = key.replace(separator, ".");
            }

            let path = Path::new(&value);
            let file = File::from(path);
            let map = file.collect();

            match map {
                Ok(map) => {
                    let uri = format!("secret:{}:{}", key, value);
                    m.insert(key, Value::new(Some(&uri), ValueKind::Table(map)));
                }
                Err(err) => {
                    error = Some(err);
                }
            }
        });

        match error {
            Some(err) => Err(err),
            None => Ok(m),
        }
    }
}
