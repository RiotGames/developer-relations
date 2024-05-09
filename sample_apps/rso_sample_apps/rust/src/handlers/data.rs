use crate::config;
use askama::Template;
use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{http, http::StatusCode, Filter, Rejection, Reply};
/// AccountData represents the account data of a user
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountData {
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

// This ChampionRotationData struct represents the champion rotation data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChampionRotationData {
    #[serde(alias = "freeChampionIds")]
    pub free_champion_ids: Vec<u32>,
    #[serde(alias = "freeChampionIdsForNewPlayers")]
    pub free_champion_ids_for_new_players: Vec<u32>,
    #[serde(alias = "maxNewPlayerLevel")]
    pub max_new_player_level: u32,
}

// Implement the Display trait for ChampionRotationData
// This allows us to print the ChampionRotationData struct in a readable format (JSON)
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
fn account_data(url: String, token: String) -> core::result::Result<AccountData, String> {
    info!("requesting account data");
    match ureq::get(url.as_str())
        .set("Authorization", format!("Bearer {token}").as_str())
        .call()
    {
        Ok(res) => {
            info!("successfully requested account data");
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
    url: String,
    token: String,
) -> core::result::Result<ChampionRotationData, String> {
    info!("requesting champion rotation data");

    match ureq::get(url.as_str())
        .set("X-Riot-Token", token.as_str())
        .call()
    {
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

/// Response struct for  the data endpoint.
/// This struct contains the account data and champion rotation data, and is serialized to JSON before being returned to the client.
#[derive(Serialize, Deserialize, Template, Clone)]
#[template(path = "data.html")]
struct Response {
    pub account: AccountData,
    pub account_data: String,
    pub champion_rotation: ChampionRotationData,
    pub champion_rotation_data: String,
    pub message: String,
}
/// Handle incoming requests for account and champion rotation data.
///
/// # Returns
///
/// A `Filter` that  handles incoming requests.
pub fn handle(
    cfg: &config::Configuration,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cfg = cfg.clone();
    warp::get()
        .and(warp::path("data"))
        .and(warp::query::<HashMap<String, String>>())
        .map(
            move |p: HashMap<String, String>| match p.get("access_token") {
                Some(access_token) => {
                    info!("☁️ handling data request");

                    debug!("☁️ requesting champion data");

                    let champion_data = match champion_rotation_data(
                        cfg.clone().champion_data_url.to_string(),
                        cfg.clone().api_token.to_string(),
                    ) {
                        Ok(champion_data) => champion_data,
                        Err(e) => {
                            debug!("☁️ error getting champion data: {e:?}");
                            return http::Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(e.to_string());
                        }
                    };
                    // Fetch champion rotation data using the provided access token. This operation may block the thread.
                    //     let champion_data = champion_rotation_data(cfg.clone().api_token.to_string())
                    //       .expect("error getting champion data");
                    debug!("☁️ received champion data: {champion_data:?}");

                    debug!("☁️ requesting account data");
                    // Fetch account data using the provided access token. This operation may block the thread.
                    let acct_data: AccountData =
                        account_data(cfg.clone().account_data_url, access_token.to_string())
                            .expect("error getting account data");

                    debug!("☁️ received account data: {acct_data:?}");

                    info!("☁️ completed handling data request");

                    // Create a `Response` object with the account and champion rotation data.
                    http::Response::builder().status(StatusCode::OK).body(
                        Response {
                            account: acct_data.clone(),
                            account_data: acct_data.clone().to_string(),
                            champion_rotation: champion_data.clone(),
                            champion_rotation_data: champion_data.clone().to_string(),
                            message: "".to_string(),
                        }
                        .to_string(),
                    )
                }
                // If no `access_token` query parameter is provided, return a response with a status code of 401
                // (Unauthorized).
                None => http::Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(String::from("unauthorized")),
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ServiceMock {
        server: MockServer,
    }

    impl ServiceMock {
        fn new(status_code: u16) -> Self {
            let server = MockServer::start();

            server.mock(|when, then| {
                 when.method(GET)
                     .path("/lol/platform/v3/champion-rotations");
                 then.status(status_code)
                     .header("content-type", "application/json")
                     .body(r#"{"freeChampionIds": [1, 2, 3], "freeChampionIdsForNewPlayers": [4, 5, 6], "maxNewPlayerLevel": 10}"#);
             });

            server.mock(|when, then| {
                when.method(GET).path("/riot/account/v1/accounts/me");
                then.status(status_code)
                    .header("content-type", "application/json")
                    .body(r#"{"puuid": "123", "gameName": "test", "tagLine": "test"}"#);
            });

            ServiceMock { server }
        }

        fn configuration(&self) -> config::Configuration {
            config::Configuration {
                server: config::Server {
                    host: "".to_string(),
                    port: 443,
                    tls: None,
                },
                api_token: "".to_string(),
                client_id: "".to_string(),
                client_secret: "".to_string(),
                provider_url: "".to_string(),
                callback_host: "".to_string(),
                account_data_url: self.server.url("/riot/account/v1/accounts/me"),
                champion_data_url: self.server.url("/lol/platform/v3/champion-rotations"),
            }
        }
    }
    use httpmock::prelude::*;

    #[test]
    fn account_data_returns_expected_result() {
        let cfg = ServiceMock::new(200).configuration();
        let result =
            account_data(cfg.account_data_url.to_string(), "test_token".to_string()).unwrap();

        assert_eq!(result.puuid, "123");
        assert_eq!(result.game_name, "test");
        assert_eq!(result.tag_line, "test");
    }

    #[test]
    fn account_data_handles_error() {
        let cfg = ServiceMock::new(500).configuration();
        let result = account_data(cfg.account_data_url.to_string(), "test_token".to_string());

        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn champion_rotation_data_returns_expected_result() {
        let cfg = ServiceMock::new(200).configuration();
        let result =
            champion_rotation_data(cfg.champion_data_url.to_string(), "test_token".to_string())
                .unwrap();

        assert_eq!(result.free_champion_ids, vec![1, 2, 3]);
        assert_eq!(result.free_champion_ids_for_new_players, vec![4, 5, 6]);
        assert_eq!(result.max_new_player_level, 10);
    }

    #[tokio::test]
    async fn champion_rotation_data_handles_error() {
        let cfg = ServiceMock::new(500).configuration();
        let result =
            champion_rotation_data(cfg.champion_data_url.to_string(), "test_token".to_string());

        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn handle_returns_expected_result() {
        let cfg = ServiceMock::new(200).configuration();
        let filter = handle(&cfg);
        let res = warp::test::request()
            .path("/data?access_token=test_token")
            .reply(&filter);

        assert_eq!(res.await.status(), 200, "Should return 200");
    }

    #[tokio::test]
    async fn handle_returns_unauthorized_when_no_access_token() {
        let cfg = ServiceMock::new(200).configuration();
        let filter = handle(&cfg);
        let res = warp::test::request().path("/data").reply(&filter);

        assert_eq!(res.await.status(), 401, "Should return 200");
    }
}
