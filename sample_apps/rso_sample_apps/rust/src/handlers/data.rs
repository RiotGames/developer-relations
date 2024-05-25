use super::HtmlTemplate;
use crate::config::Configuration;
use askama::Template;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};

/// AccountData represents the account data of a user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountData {
    pub puuid: String,
    #[serde(alias = "gameName")]
    pub game_name: String,
    #[serde(alias = "tagLine")]
    pub tag_line: String,
}

/// Implement the Display trait for AccountData
/// This allows us to print the AccountData struct in a readable format
impl std::fmt::Display for AccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let j = serde_json::to_string(&self).expect("error serializing json");
        write!(f, "{}", j)
    }
}

/// This ChampionRotationData struct represents the champion rotation data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChampionRotationData {
    #[serde(alias = "freeChampionIds")]
    pub free_champion_ids: Vec<usize>,
    #[serde(alias = "freeChampionIdsForNewPlayers")]
    pub free_champion_ids_for_new_players: Vec<usize>,
    #[serde(alias = "maxNewPlayerLevel")]
    pub max_new_player_level: usize,
}

/// Implement the Display trait for ChampionRotationData
/// This allows us to print the ChampionRotationData struct in a readable format (JSON)
impl std::fmt::Display for ChampionRotationData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let j = serde_json::to_string(&self).expect("error serializing json");
        write!(f, "{}", j)
    }
}

/// Fetches the account data for a user.
///
/// This function sends a GET request to the provided URL with the provided token as the Authorization header.
/// If the request is successful, it deserializes the response into an `AccountData` struct and returns it.
/// If the request fails, it logs the error and returns a string describing the error.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL for the request.
/// * `token` - A string slice that holds the Authorization token for the request.
///
/// # Returns
///
/// This function returns a `Result` that contains an `AccountData` struct if the request is successful,
/// or a string describing the error if the request fails.
///
/// # Example
///
/// ```
/// let url = "https://americas.api.riotgames.com/riot/account/v1/accounts/me";
/// let token = "my_token";
/// let account_data = account_data(url, token);
/// match account_data {
///     Ok(data) => println!("Account data: {:?}", data),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
fn account_data(url: &str, token: &str) -> core::result::Result<AccountData, String> {
    debug!("requesting account data");
    match ureq::get(url)
        .set("Authorization", format!("Bearer {token}").as_str())
        .call()
    {
        Ok(res) => {
            debug!("successfully requested account data");
            Ok(serde_json::from_str(res.into_string().unwrap().as_mut_str()).unwrap())
        }
        Err(e) => {
            error!("error getting account data: {e}");
            Err(e.to_string())
        }
    }
}

/// Fetches the champion rotation data.
///
/// This function sends a GET request to the provided URL with the provided token as the `X-Riot-Token` header.
/// If the request is successful, it deserializes the response into a `ChampionRotationData` struct and returns it.
/// If the request fails, it logs the error and returns a string describing the error.
///
/// # Arguments
///
/// * `url` - A `String` that holds the URL for the request.
/// * `token` - A `String` that holds the `X-Riot-Token` for the request.
///
/// # Returns
///
/// This function returns a `Result` that contains a `ChampionRotationData` struct if the request is successful,
/// or a `String` describing the error if the request fails.
///
/// # Example
///
/// ```
/// let url = "https://na1.api.riotgames.com/lol/platform/v3/champion-rotations";
/// let token = "my_token";
/// let champion_rotation_data = champion_rotation_data(url, token);
/// match champion_rotation_data {
///     Ok(data) => println!("Champion rotation data: {:?}", data),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
fn champion_rotation_data(
    url: &str,
    token: &str,
) -> core::result::Result<ChampionRotationData, String> {
    info!("requesting champion rotation data");
    match ureq::get(url).set("X-Riot-Token", token).call() {
        Ok(res) => {
            info!("successfully requested champion rotation data");
            Ok(serde_json::from_str(res.into_string().unwrap().as_mut_str()).unwrap())
        }
        Err(e) => {
            error!("error getting champion data: {e}");
            Err(e.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// An OAuth request containing a code.
pub struct Request {
    /// The code that was given to us after the user authenticated with the
    /// provider.
    pub access_token: String,
}

/// Response struct for  the data endpoint.
/// This struct contains the account data and champion rotation data, and is serialized to JSON before being returned to the client.
#[derive(Serialize, Deserialize, Template, Clone)]
#[template(path = "data.html")]
pub struct Response {
    pub account: AccountData,
    pub account_data: String,
    pub champion_rotation: ChampionRotationData,
    pub champion_rotation_data: String,
    pub message: String,
}

// basic handler that responds with a static string
pub async fn handle(
    Query(query): Query<Request>,
    State(cfg): State<Configuration>,
) -> impl IntoResponse {
    if query.access_token.is_empty() {
        return Err("unauthorized".to_string());
    }

    info!("☁️ handling data request");

    // Fetch champion rotation data using the provided access token. This operation may block the thread.
    let champion_data = champion_rotation_data(&cfg.champion_data_url, &cfg.api_token)
        .map_err(|e| format!("{:?}", e))?;

    // Fetch account data using the provided access token. This operation may block the thread.
    let acct_data =
        account_data(&cfg.account_data_url, &query.access_token).map_err(|e| format!("{:?}", e))?;

    info!("☁️ completed handling data request");

    // Create a `Response` object with the account and champion rotation data.
    Ok(HtmlTemplate(Response {
        account: acct_data.clone(),
        account_data: acct_data.clone().to_string(),
        champion_rotation: champion_data.clone(),
        champion_rotation_data: champion_data.clone().to_string(),
        message: "".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use crate::config::Configuration;
    use crate::handlers::data::{account_data, champion_rotation_data};

    #[test]
    fn account_data_returns_expected_result() {
        let api = mock::ApiProvider::new();
        let cfg = Configuration {
            server: crate::config::Server {
                host: "".to_string(),
                port: 443,
                tls: None,
            },
            api_token: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            callback_host: "".to_string(),
            account_data_url: api.server.url("/riot/account/v1/accounts/me").to_string(),
            champion_data_url: "".to_string(),
            provider_url: "".to_string(),
        };
        let res = account_data(&cfg.account_data_url, "token");
        assert_eq!(false, res.is_err());
    }

    #[test]
    fn account_data_handles_error() {
        let api = mock::ApiProvider::new();
        let cfg = Configuration {
            server: crate::config::Server {
                host: "".to_string(),
                port: 443,
                tls: None,
            },
            api_token: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            callback_host: "".to_string(),
            account_data_url: api.server.url("/riot/account/v1/accounts/me").to_string(),
            champion_data_url: "".to_string(),
            provider_url: "".to_string(),
        };
        let res = account_data(&cfg.account_data_url, "");
        assert_eq!(true, res.is_err());
    }

    #[tokio::test]
    async fn champion_rotation_data_returns_expected_result() {
        let api = mock::ApiProvider::new();
        let cfg = Configuration {
            server: crate::config::Server {
                host: "".to_string(),
                port: 443,
                tls: None,
            },
            api_token: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            callback_host: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: api
                .server
                .url("/lol/platform/v3/champion-rotations")
                .to_string(),
            provider_url: "".to_string(),
        };
        let res = champion_rotation_data(&cfg.champion_data_url, "token");
        assert_eq!(false, res.is_err());
    }

    #[tokio::test]
    async fn champion_rotation_data_handles_error() {
        let api = mock::ApiProvider::new();
        let cfg = Configuration {
            server: crate::config::Server {
                host: "".to_string(),
                port: 443,
                tls: None,
            },
            api_token: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            callback_host: "".to_string(),
            account_data_url: "".to_string(),
            champion_data_url: api
                .server
                .url("/lol/platform/v3/champion-rotations")
                .to_string(),
            provider_url: "".to_string(),
        };
        let res = champion_rotation_data(&cfg.account_data_url, "");
        assert_eq!(true, res.is_err());
    }
}
