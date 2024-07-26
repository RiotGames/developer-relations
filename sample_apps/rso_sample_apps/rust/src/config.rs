use config::{Config, Environment, File};
use log::debug;
use serde_derive::{Deserialize, Serialize};

/// Represents the TLS configuration for the server.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Tls {
    /// The TLS certificate
    pub cert: String,
    /// The TLS key
    pub key: String,
}

/// Configuration for the server, including address and optional TLS settings.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Server {
    /// The server's address in the format `ip:port`.
    pub addr: String,
    /// Optional TLS configuration for secure connections.
    pub tls: Option<Tls>,
}

/// Main configuration structure for the application.
///
/// Holds configurations for the server, OAuth client, and API endpoints.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configuration {
    /// Server configuration, including address and TLS settings.
    pub server: Server,
    /// OAuth client configuration.
    pub rso: Rso,
    /// API endpoint configurations.
    pub api: Api,
}

impl From<config::Config> for Configuration {
    fn from(cfg: config::Config) -> Self {
        Configuration {
            server: Server {
                addr: cfg.get::<String>("SERVER_ADDRESS").unwrap_or_default(),
                tls: cfg.get::<Option<Tls>>("SERVER_TLS").unwrap_or(None),
            },
            rso: Rso {
                base_url: cfg.get::<String>("RSO_BASE_URL").unwrap_or_default(),
                callback_host: cfg.get::<String>("RSO_CALLBACK_HOST").unwrap_or_default(),
                client_id: cfg.get::<String>("RSO_CLIENT_ID").unwrap_or_default(),
                client_secret: cfg.get::<String>("RSO_CLIENT_SECRET").unwrap_or_default(),
            },
            api: Api {
                token: cfg.get::<String>("RGAPI_TOKEN").unwrap_or_default(),
                urls: Urls {
                    account_data: cfg
                        .get::<String>("RGAPI_URL_ACCOUNT_DATA")
                        .unwrap_or_default(),
                    champion_data: cfg
                        .get::<String>("RGAPI_URL_CHAMPION_DATA")
                        .unwrap_or_default(),
                },
            },
        }
    }
}

/// OAuth client configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Rso {
    /// Base URL for the OAuth provider.
    pub base_url: String,
    /// Host to which the OAuth provider will redirect after authentication.
    pub callback_host: String,
    /// Client ID for OAuth authentication.
    pub client_id: String,
    /// Client secret for OAuth authentication.
    pub client_secret: String,
}

/// Configuration for API endpoints.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Api {
    /// Token for API authentication.
    pub token: String,
    /// URLs for different API endpoints.
    pub urls: Urls,
}

/// URLs for the API endpoints.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Urls {
    /// Endpoint for retrieving account data.
    pub account_data: String,
    /// Endpoint for retrieving champion data.
    pub champion_data: String,
}

impl Configuration {
    /// Constructs the callback URL for OAuth provider redirection.
    ///
    /// # Returns
    /// A `String` representing the full callback URL.
    pub fn callback_url(&self) -> String {
        let protocol = match self.server.tls {
            Some(_) => "https://",
            None => "http://",
        };
        format!("{}{}/oauth", protocol, self.rso.callback_host)
    }

    /// Constructs the token endpoint URL.
    ///
    /// # Returns
    /// A `String` representing the full token endpoint URL.
    pub fn token_url(&self) -> String {
        format!("{}/token", self.rso.base_url)
    }

    /// Constructs the authorization endpoint URL.
    ///
    /// # Returns
    /// A `String` representing the full authorization endpoint URL.
    pub fn authorize_url(&self) -> String {
        format!("{}/authorize", self.rso.base_url)
    }

    /// Constructs the sign-in URL with query parameters for OAuth authentication.
    ///
    /// # Returns
    /// A `String` representing the full sign-in URL.
    pub fn sign_in_url(&self) -> String {
        format!(
            "{}?redirect_uri={}&client_id={}&response_type=code&scope=openid",
            self.authorize_url(),
            self.callback_url(),
            self.rso.client_id,
        )
    }
}

/// Parses the application configuration from a file and environment variables.
///
/// # Arguments
/// * `filepath` - The path to the configuration file.
///
/// # Returns
/// A `Result` which is `Ok` with the `Configuration` if parsing and deserialization are successful,
/// or an `Err` with a string message indicating the error.
pub(crate) fn parse(filepath: String) -> Result<Configuration, String> {
    let cfg = match Config::builder()
        .add_source(File::with_name(&filepath).required(false))
        .add_source(Environment::default())
        .build()
    {
        Ok(cfg) => cfg,
        Err(e) => {
            return Err(format!("error parsing configuration - {e}"));
        }
    };

    match cfg.try_deserialize() {
        Ok(cfg) => {
            debug!("📄 parsed configuration successfully");
            Ok(cfg)
        }
        Err(e) => Err(format!("error deserializing configuration - {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::config::Tls;
    use std::collections::HashMap;

    fn create_cfg() -> Configuration {
        Configuration {
            server: config::Server {
                addr: "0.0.0.0:443".to_string(),
                tls: None,
            },
            api: Api {
                token: "token".to_string(),
                urls: Urls {
                    account_data: "account_data".to_string(),
                    champion_data: "champion_data".to_string(),
                },
            },
            rso: Rso {
                base_url: "base_url".to_string(),
                callback_host: "local.example.com:8080".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
        }
    }

    fn create_cfg_tls() -> Configuration {
        Configuration {
            server: config::Server {
                addr: "0.0.0.0:443".to_string(),
                tls: Some(Tls {
                    cert: "cert".to_string(),
                    key: "key".to_string(),
                }),
            },
            api: Api {
                token: "token".to_string(),
                urls: Urls {
                    account_data: "account_data".to_string(),
                    champion_data: "champion_data".to_string(),
                },
            },
            rso: Rso {
                base_url: "base_url".to_string(),
                callback_host: "local.example.com:8080".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
        }
    }

    #[test]
    fn test_config_env() {
        let mut env = HashMap::new();
        let keys_and_values = [
            ("SERVER_ADDRESS", "SERVER_ADDRESS"),
            ("SERVER_TLS_CERT", "SERVER_TLS_CERT"),
            ("SERVER_TLS_KEY", "SERVER_TLS_KEY"),
            ("RSO_BASE_URL", "RSO_BASE_URL"),
            ("RSO_CALLBACK_HOST", "RSO_CALLBACK_HOST"),
            ("RSO_CLIENT_ID", "RSO_CLIENT_ID"),
            ("RSO_CLIENT_SECRET", "RSO_CLIENT_SECRET"),
            ("RGAPI_TOKEN", "RGAPI_TOKEN"),
            ("RGAPI_URL_ACCOUNT_DATA", "RGAPI_URL_ACCOUNT_DATA"),
            ("RGAPI_URL_CHAMPION_DATA", "RGAPI_URL_CHAMPION_DATA"),
        ];

        for (key, value) in keys_and_values.iter() {
            env.insert((*key).into(), (*value).into());
        }

        let source = Environment::default().source(Some(env));
        let c: Configuration = Config::builder()
            .add_source(source)
            .build()
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(c.server.addr, "SERVER_ADDRESS");
        assert_eq!(c.rso.base_url, "RSO_BASE_URL");
        assert_eq!(c.rso.callback_host, "RSO_CALLBACK_HOST");
        assert_eq!(c.rso.client_id, "RSO_CLIENT_ID");
        assert_eq!(c.rso.client_secret, "RSO_CLIENT_SECRET");
        assert_eq!(c.api.token, "RGAPI_TOKEN");
        assert_eq!(c.api.urls.account_data, "RGAPI_URL_ACCOUNT_DATA");
        assert_eq!(c.api.urls.champion_data, "RGAPI_URL_CHAMPION_DATA");
    }

    #[test]
    fn test_sign_in_url() {
        let config = create_cfg();
        assert_eq!(
            config.sign_in_url(),
            "base_url/authorize?redirect_uri=http://local.example.com:8080/oauth&client_id=client_id&response_type=code&scope=openid",
        );
    }

    #[test]
    fn test_sign_in_url_tls() {
        let config = create_cfg_tls();
        assert_eq!(
            config.sign_in_url(),
            "base_url/authorize?redirect_uri=https://local.example.com:8080/oauth&client_id=client_id&response_type=code&scope=openid",
        );
    }

    #[test]
    fn test_callback_url() {
        let config = create_cfg();
        assert_eq!(config.callback_url(), "http://local.example.com:8080/oauth");
    }

    #[test]
    fn test_callback_url_tls() {
        let config = create_cfg_tls();
        assert_eq!(
            config.callback_url(),
            "https://local.example.com:8080/oauth"
        );
    }

    #[test]
    fn test_token_url() {
        let config = create_cfg();
        assert_eq!(config.token_url(), "base_url/token");
    }

    #[test]
    fn test_authorize_url() {
        let config = create_cfg();
        assert_eq!(config.authorize_url(), "base_url/authorize");
    }

    #[test]
    fn server_with_tls() {
        let server = Server {
            addr: "0.0.0.0:443".to_string(),
            tls: Some(Tls {
                cert: "cert".to_string(),
                key: "key".to_string(),
            }),
        };

        assert!(server.tls.is_some());
    }

    #[test]
    fn server_without_tls() {
        let server = Server {
            addr: "0.0.0.0:443".to_string(),
            tls: None,
        };

        assert!(server.tls.is_none());
    }

    #[test]
    fn tls_struct_holds_certificate_and_key() {
        let tls = Tls {
            cert: "certificate".to_string(),
            key: "key".to_string(),
        };

        assert_eq!(tls.cert, "certificate");
        assert_eq!(tls.key, "key");
    }
}
