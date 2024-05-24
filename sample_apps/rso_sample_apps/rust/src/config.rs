use config::{Config, Environment, File};
use serde_derive::{Deserialize, Serialize};
use log::{debug};

/// The server TLS configuration
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Tls {
    /// The TLS certificate
    pub cert: String,
    /// The TLS key
    pub key: String,
}

/// The server configuration
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Server {
    /// The server hostname
    pub host: String,
    // The port
    pub port: u16,
    /// The server TLS configuration
    pub tls: Option<Tls>,
}

impl Server {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Configuration holds the  required parameters to connect to a Git provider.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configuration {
    /// The  client ID.
    pub client_id: String,
    /// The client secret.
    pub client_secret: String,
    /// The provider URL.
    pub provider_url: String,
    /// The server configuration.
    pub server: Server,
    /// The callback host.
    pub callback_host: String,
    /// The RiotGamesAPI Token
    pub api_token: String,
    /// The account data URL
    pub account_data_url: String,
    /// The champion data URL
    pub champion_data_url: String,
}

impl Configuration {
    /// The callback URL that the OAuth provider will redirect to after authentication.
    pub fn callback_url(&self) -> String {
        let protocol = match self.server.tls {
            Some(_) => "https://",
            None => "http://",
        };
        format!("{}{}/oauth", protocol, self.callback_host)
    }

    /// Returns the provide token endpoint.
    pub fn token_url(&self) -> String {
        format!("{}/token", self.provider_url)
    }

    /// Returns the URL of the OAuth authorize endpoint.
    ///
    ///  The authorize endpoint is where the user is redirected to grant the application permission to
    /// access their data. The user will be prompted to log in to their account  and grant the application
    /// the requested permissions. Once the user has granted the permissions, they will be redirected
    /// back to the application with an authorization code.
    pub fn authorize_url(&self) -> String {
        format!("{}/authorize", self.provider_url)
    }

    /// Returns the  sign in URL which opens the browser authenticates, and redirects to the
    /// callback URL to complete authentication.
    pub fn sign_in_url(&self) -> String {
        format!(
            "{}?redirect_uri={}&client_id={}&response_type=code&scope=openid",
            self.authorize_url(),
            self.callback_url(),
            self.client_id,
        )
    }
}

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
        },
        Err(e) => Err(format!("error deserializing configuration - {e}")),
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::config::Tls;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_sign_in_url() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_ secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: None,
            },
            callback_host: "localhost:8080".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(
            config.sign_in_url(),
            "provider_url/authorize?redirect_uri=http://localhost:8080/oauth&client_id=client_id&response_type=code&scope=openid",
        );
    }

    #[test]
    fn test_sign_in_url_tls() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_ secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: Some(Tls {
                    cert: "cert".to_string(),
                    key: "key".to_string(),
                }),
            },
            callback_host: "localhost:8080".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(
            config.sign_in_url(),
            "provider_url/authorize?redirect_uri=https://localhost:8080/oauth&client_id=client_id&response_type=code&scope=openid",
        );
    }

    #[test]
    fn test_callback_url() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: None,
            },
            callback_host: "localhost:8080".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(config.callback_url(), "http://localhost:8080/oauth");
    }

    #[test]
    fn test_callback_url_tls() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: Some(Tls {
                    cert: "cert".to_string(),
                    key: "key".to_string(),
                }),
            },
            callback_host: "localhost:8080".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(config.callback_url(), "https://localhost:8080/oauth");
    }

    #[test]
    fn test_token_url() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: None,
            },
            callback_host: "".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(config.token_url(), "provider_url/token");
    }

    #[test]
    fn test_authorize_url() {
        let config = config::Configuration {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            provider_url: "provider_url".to_string(),
            server: config::Server {
                host: "localhost".to_string(),
                port: 8080,
                tls: None,
            },
            callback_host: "".to_string(),
            api_token: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: "".to_string(),
        };
        assert_eq!(config.authorize_url(), "provider_url/authorize");
    }

    #[test]
    fn server_addr_combines_host_and_port() {
        let server = Server {
            host: "localhost".to_string(),
            port: 8080,
            tls: None,
        };

        assert_eq!(server.addr(), "localhost:8080");
    }

    #[test]
    fn server_with_tls() {
        let server = Server {
            host: "localhost".to_string(),
            port: 8080,
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
            host: "localhost".to_string(),
            port: 8080,
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
