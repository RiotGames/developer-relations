# RUST RSO Sample app

#### Description

### Requirements
1. Make sure you have your `config.yml` in the root of the project [(_example_)](config/config.yml): `sample_apps/rso_sample_apps/rust/config.yml`.
1. You need rust 1.77.2 installed on your machine. You can install it from [here](https://www.rust-lang.org/tools/install).

---
### Configuration

```yaml
server: # server configuration ,
  addr: 0.0.0.0:443 # SERVER_ADDRESS - address to bind to
  tls:
    cert: # SERVER_TLS_CERT - tls certificate
    key: # SERVER_TLS_KEY - tls private key

rso:
  base_url: "https://auth.riotgames.com" # RSO_BASE_URL - Riot Games oauth auth provider URL
  callback_host: "https://local.example.com" # RSO_CALLBACK_HOST - callback hostname
  client_id: "" # RSO_CLIENT_ID - oauth2 client id
  client_secret: "" # RSO_CLIENT_SECRET - oauth2 client secret

api:
  token: "" # Riot Games API token - RGAPI_TOKEN
  urls:
    account_data: "https://americas.api.riotgames.com/riot/account/v1/accounts/me" # RGAPI_URL_ACCOUNT_DATA - Riot Games API account data URL
    champion_data: "https://na1.api.riotgames.com/lol/platform/v3/champion-rotations" # RGAPI_URL_CHAMPION_DATA - Riot Games API champion data URL
```
#### API Token
_Used to access most of the Riot Games API endpoints. [More Information](https://developer.riotgames.com/docs/portal#web-apis_api-keys)_ 
 
- **api:**
  - **token:** Your Riot Games API Token 

#### OAuth/RSO Configuration
_Used to authenticate users with the Riot Games services requiring OAuth2/RSO authentication. [More Information](https://developer.riotgames.com/docs/lol#rso-integration)._

- **rso:**
  - **client_id:** OAuth2 client id
  - **client_secret:** OAuth2 client secret

---
### Makefile

This Makefile contains several commands that help with building, testing, cleaning, and running the Rust project.

#### Commands

- `make all`: This command first cleans up any previous build artifacts, then builds the project and runs the tests. It's a quick way to ensure everything is up to date and working correctly.

- `make build`: This command builds the Rust project. It compiles the source code into an executable file.

- `make test`: This command runs the tests for the Rust project. It ensures that all the functions in the project are working as expected.

- `make clean`: This command cleans up any build artifacts from previous builds. It's a good practice to clean up before starting a new build.

- `make run`: This command runs the Rust project. It starts the application.

- `make debug` : This command runs the Rust project with debug logging enabled. It starts the application.

To use these commands, open a terminal in the project's root directory and type the command you want to use. For example, to build the project, type `make build`.
