use httpmock::prelude::*;

pub struct AuthProvider {
    pub server: MockServer,
}

impl AuthProvider {
    pub fn new() -> Self {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(POST)
                .path("/token")
                .x_www_form_urlencoded_key_exists("code");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"access_token": "xyz", "refresh_token": "abc", "scope": "def", "id_token": "ghi", "token_type": "jkl", "expires_in": 3600}"#);
        });

        server.mock(|when, then| {
            when.method(POST).path("/token");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{}"#);
        });

        AuthProvider { server }
    }
}

pub struct ApiProvider {
   pub server: MockServer,
}

impl ApiProvider {
    pub fn new() -> Self {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET)
                .path("/riot/account/v1/accounts/me")
                .header("Authorization", "Bearer token");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"puuid":"123","game_name":"user","tag_line":"tag"}"#);
        });

        server.mock(|when, then| {
            when.method(GET).path("/riot/account/v1/accounts/me");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{}"#);
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/lol/platform/v3/champion-rotations")
                .header("X-Riot-Token", "token");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"free_champion_ids":[1,2,3],"free_champion_ids_for_new_players":[100,101,102],"max_new_player_level":10}"#);
        });

        server.mock(|when, then| {
            when.method(GET).path("/lol/platform/v3/champion-rotations");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{}"#);
        });

        ApiProvider { server }
    }
}
