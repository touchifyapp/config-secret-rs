use config::{Config, Source};
use config_secret::EnvironmentSecretFile;

mod helpers;
use crate::helpers::{get_test_file, ScopedSettings, Settings};

/// Reminder that tests using env variables need to use different env variable names, since
/// tests can be run in parallel

#[test]
fn test_prefix_is_removed_from_key() {
    temp_env::with_var("A_B_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("A");
        assert!(source.collect().unwrap().contains_key("b"));
    })
}

#[test]
fn test_prefix_with_variant_forms_of_spelling() {
    temp_env::with_var("a_A_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("a");
        assert!(source.collect().unwrap().contains_key("a"));
    });

    temp_env::with_var("aB_A_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("aB");
        assert!(source.collect().unwrap().contains_key("a"));
    });

    temp_env::with_var("Ab_A_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("ab");
        assert!(source.collect().unwrap().contains_key("a"));
    });
}

#[test]
fn test_separator_behavior() {
    temp_env::with_var("C_B_A_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("C").separator("_");
        assert!(source.collect().unwrap().contains_key("b.a"));
    })
}

#[test]
fn test_empty_value_is_ignored() {
    temp_env::with_var("C_A_B_FILE", Some(""), || {
        let source = EnvironmentSecretFile::with_prefix("c");
        assert!(!source.collect().unwrap().contains_key("a_b"));
    })
}

#[test]
fn test_keep_prefix() {
    temp_env::with_var("C_A_C_FILE", Some(get_test_file("config.json")), || {
        // Do not keep the prefix
        let source = EnvironmentSecretFile::with_prefix("C");
        assert!(source.collect().unwrap().contains_key("a_c"));

        let source = EnvironmentSecretFile::with_prefix("C").keep_prefix(false);
        assert!(source.collect().unwrap().contains_key("a_c"));

        // Keep the prefix
        let source = EnvironmentSecretFile::with_prefix("C").keep_prefix(true);
        assert!(source.collect().unwrap().contains_key("c_a_c"));
    })
}

#[test]
fn test_custom_separator_behavior() {
    temp_env::with_var("C.B.A.FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("C").separator(".");
        assert!(source.collect().unwrap().contains_key("b.a"));
    })
}

#[test]
fn test_custom_prefix_separator_behavior() {
    temp_env::with_var("C-B.A.FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("C")
            .separator(".")
            .prefix_separator("-");

        assert!(source.collect().unwrap().contains_key("b.a"));
    })
}

#[test]
fn test_custom_suffix_behavior() {
    temp_env::with_var("C_B_A_SECRET", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("C")
            .separator("_")
            .suffix("SECRET");

        assert!(source.collect().unwrap().contains_key("b.a"));
    })
}

#[test]
fn test_custom_suffix_separator_behavior() {
    temp_env::with_var("C.B.A-FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("C")
            .separator(".")
            .suffix_separator("-");

        assert!(source.collect().unwrap().contains_key("b.a"));
    })
}

#[test]
fn test_any_format_behavior() {
    temp_env::with_var("D_E_F_FILE", Some(get_test_file("config.yaml")), || {
        let source = EnvironmentSecretFile::with_prefix("D").separator("_");
        assert!(source.collect().unwrap().contains_key("e.f"));
    })
}

#[test]
fn test_full_pattern_behavior() {
    temp_env::with_var("F_FILE", Some(get_test_file("config.yaml")), || {
        let source = EnvironmentSecretFile::with_prefix("F").separator("_");
        assert!(source.collect().unwrap().contains_key("server"));
        assert!(source.collect().unwrap().contains_key("redis"));
    })
}

#[test]
fn test_scoped_serialize() {
    temp_env::with_var("J_A_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("J").separator("_");

        let config = Config::builder().add_source(source).build().unwrap();

        let settings = config.try_deserialize::<ScopedSettings>().unwrap();

        assert!(settings.a.server.host == "0.0.0.0");
        assert!(settings.a.server.port == 5000);
        assert!(
            settings.a.redis.nodes
                == vec![
                    "redis://10.0.0.1:6379",
                    "redis://10.0.0.2:6379",
                    "redis://10.0.0.3:6379"
                ]
        );
    })
}

#[test]
fn test_scoped_serialize_yaml() {
    temp_env::with_var("Y_A_FILE", Some(get_test_file("config.yaml")), || {
        let source = EnvironmentSecretFile::with_prefix("Y").separator("_");

        let config = Config::builder().add_source(source).build().unwrap();

        let settings = config.try_deserialize::<ScopedSettings>().unwrap();

        assert!(settings.a.server.host == "0.0.0.0");
        assert!(settings.a.server.port == 5000);
        assert!(
            settings.a.redis.nodes
                == vec![
                    "redis://10.0.0.1:6379",
                    "redis://10.0.0.2:6379",
                    "redis://10.0.0.3:6379"
                ]
        );
    })
}

#[test]
fn test_full_serialize() {
    temp_env::with_var("F_FILE", Some(get_test_file("config.json")), || {
        let source = EnvironmentSecretFile::with_prefix("F").separator("_");

        let config = Config::builder().add_source(source).build().unwrap();

        let settings = config.try_deserialize::<Settings>().unwrap();

        assert!(settings.server.host == "0.0.0.0");
        assert!(settings.server.port == 5000);
        assert!(
            settings.redis.nodes
                == vec![
                    "redis://10.0.0.1:6379",
                    "redis://10.0.0.2:6379",
                    "redis://10.0.0.3:6379"
                ]
        );
    })
}

#[test]
fn test_full_serialize_yaml() {
    temp_env::with_var("FY_FILE", Some(get_test_file("config.yaml")), || {
        let source = EnvironmentSecretFile::with_prefix("FY").separator("_");

        let config = Config::builder().add_source(source).build().unwrap();

        let settings = config.try_deserialize::<Settings>().unwrap();

        assert!(settings.server.host == "0.0.0.0");
        assert!(settings.server.port == 5000);
        assert!(
            settings.redis.nodes
                == vec![
                    "redis://10.0.0.1:6379",
                    "redis://10.0.0.2:6379",
                    "redis://10.0.0.3:6379"
                ]
        );
    })
}
