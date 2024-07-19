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
/// * `url` - A string slice that holds the URL for the request.
/// * `token` - A string slice that holds the Authorization token for the request.
///
/// # Returns
/// This function returns a `Result` that contains an `AccountData` struct if the request is successful,
/// or a string describing the error if the request fails.
///
/// # Example
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
/// * `url` - A `String` that holds the URL for the request.
/// * `token` - A `String` that holds the `X-Riot-Token` for the request.
///
/// # Returns
/// This function returns a `Result` that contains a `ChampionRotationData` struct if the request is successful,
/// or a `String` describing the error if the request fails.
///
/// # Example
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
    debug!("requesting champion rotation data");
    match ureq::get(url).set("X-Riot-Token", token).call() {
        Ok(res) => {
            debug!("successfully requested champion rotation data");
            Ok(serde_json::from_str(res.into_string().unwrap().as_mut_str()).unwrap())
        }
        Err(e) => {
            error!("error getting champion data: {e}");
            Err(e.to_string())
        }
    }
}

/// Represents a request containing an access token.
///
/// This struct is used to deserialize requests where the client provides an access token
/// obtained after authenticating with an OAuth provider. The access token is then used
/// to authorize requests to protected resources.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    /// The access token that was given to us after the user authenticated with the
    /// provider. This token is used to authenticate requests made to the API.
    pub access_token: String,
}

/// Represents the response sent to the client for data requests.
///
/// This struct contains both account data and champion rotation data, serialized into JSON
/// before being returned to the client. It is used as the response body for requests to the
/// data endpoint, providing the client with the requested information in a structured format.
#[derive(Serialize, Deserialize, Template, Clone)]
#[template(path = "data.html")]
pub struct Response {
    /// The account data retrieved for the user.
    /// Contains information such as the player's unique identifier, game name, and tag line.
    pub account: AccountData,
    /// A serialized JSON string of the account data.
    /// This is a string representation of the `account` field, provided for convenience.
    pub account_data: String,
    /// The champion rotation data retrieved from the game server.
    /// Contains information about the current set of champions available for free play.
    pub champion_rotation: ChampionRotationData,
    /// A serialized JSON string of the champion rotation data.
    /// This is a string representation of the `champion_rotation` field, provided for convenience.
    pub champion_rotation_data: String,
    /// An optional message that can be included in the response.
    /// This field can be used to convey additional information to the client, such as error messages or notices.
    pub message: String,
}

/// Handles data requests by fetching account and champion rotation data.
///
/// This asynchronous function acts as a handler for incoming data requests. It first checks if the provided
/// access token is empty, returning an error if so. If the access token is present, it proceeds to fetch
/// both account data and champion rotation data using the provided access token and configuration settings.
/// Upon successful retrieval of both data sets, it constructs a `Response` object containing the fetched data
/// and returns it wrapped in an `HtmlTemplate` for rendering.
///
/// # Arguments
/// * `query` - Extracted query parameters from the request, containing the access token.
/// * `cfg` - Application configuration state, containing URLs and tokens for data fetching.
///
/// # Returns
/// A result wrapped in `impl IntoResponse`, which on success contains an `HtmlTemplate<Response>` with the fetched data,
/// or an error string if the access token is missing or data fetching fails.
///
/// # Errors
/// Returns an error if the access token is empty or if there is an issue fetching the account or champion rotation data.
///
pub async fn handle(
    Query(query): Query<Request>,
    State(cfg): State<Configuration>,
) -> impl IntoResponse {
    if query.access_token.is_empty() {
        return Err("unauthorized".to_string());
    }

    info!("☁️ handling data request");

    // Fetch champion rotation data using the provided access token. This operation may block the thread.
    let champion_data = champion_rotation_data(&cfg.api.urls.champion_data, &cfg.api.token)
        .map_err(|e| format!("{:?}", e))?;

    // Fetch account data using the provided access token. This operation may block the thread.
    let acct_data = account_data(&cfg.api.urls.account_data, &query.access_token)
        .map_err(|e| format!("{:?}", e))?;

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
    use crate::config::{Api, Configuration, Rso, Tls, Urls};
    use crate::handlers::data::{account_data, champion_rotation_data};

    fn create_cfg_api_url(url: String) -> Configuration {
        Configuration {
            server: crate::config::Server {
                addr: "0.0.0.0:443".to_string(),
                tls: Some(Tls {
                    cert: "cert".to_string(),
                    key: "key".to_string(),
                }),
            },
            api: Api {
                token: "token".to_string(),
                urls: Urls {
                    account_data: url.clone(),
                    champion_data: url.clone(),
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
    fn account_data_returns_expected_result() {
        let api = mock::ApiProvider::new();
        let cfg = create_cfg_api_url(api.server.url("/riot/account/v1/accounts/me").to_string());
        let res = account_data(&cfg.api.urls.account_data, "token");
        assert_eq!(false, res.is_err());
    }

    #[test]
    fn account_data_handles_error() {
        let api = mock::ApiProvider::new();
        let cfg = create_cfg_api_url(api.server.url("/riot/account/v1/accounts/me").to_string());
        let res = account_data(&cfg.api.urls.account_data, "");

        assert_eq!(true, res.is_err());
    }

    #[tokio::test]
    async fn champion_rotation_data_returns_expected_result() {
        let api = mock::ApiProvider::new();
        let cfg = create_cfg_api_url(
            api.server
                .url("/lol/platform/v3/champion-rotations")
                .to_string(),
        );
        let res = champion_rotation_data(&cfg.api.urls.champion_data, "token");
        assert_eq!(false, res.is_err());
    }

    #[tokio::test]
    async fn champion_rotation_data_handles_error() {
        let api = mock::ApiProvider::new();
        let cfg = create_cfg_api_url(
            api.server
                .url("/lol/platform/v3/champion-rotations")
                .to_string(),
        );
        let res = champion_rotation_data(&cfg.api.urls.champion_data, "");
        assert_eq!(true, res.is_err());
    }
}
