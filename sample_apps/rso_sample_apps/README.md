# samples
The goal of this repository is to show you how quickly you can start creating your own backend service that implements your Riot approved app.

This repo contains samples of the most popular programming languages:

* python 🐍
* nodejs 🐢
* golang 🦫

# ⚠️ Warning ⚠️
These samples are not suited for production environments, this samples are intended to show how RSO can be used to consume Riot API endpoints.

# Requirements to run the samples ✅
* Get your secrets:
    * RSO_BASE_URL=https://auth.riotgames.com
    * RSO_CLIENT_ID=<your-client-id>
    * RSO_CLIENT_SECRET=<your-client-secret>
    * DEPLOYMENT_API_KEY=<your-deployment-api-key>
    * APP_BASE_URL=http://<your-app-base-url>
    * APP_CALLBACK_PATH=<your-callback-path>
    * RESPONSE_TYPE=code
    * SCOPE=openid
* Make sure you have your `.env` in the root of the project: `sample_apps/rso_sample_apps/.env` env file is shared between sample apps (go, python, nodejs)
* Add `127.0.0.1       local.exampleapp.com` to your hosts file.
    * MacOS/Linux
    * Append `127.0.0.1       local.exampleapp.com` to the file `/etc/hosts`  
    * Windows
        * Append `127.0.0.1       local.exampleapp.com` to the file `C:\Windows\System32\Drivers\etc\hosts`  
* Follow the README.md inside every sample app, every sample app.
* Once your app is up and running click the following link to visualize it:
    * http://local.exampleapp.com:3000

# Environment variables ☀️
Sample apps use dotenv to load the variables specified in `.env` in your environment.

If you dont want to use dotenv you can set your environment vars manually.

Sample `.env` file
```
RSO_BASE_URL=https://auth.riotgames.com
RSO_CLIENT_ID=riot-example-app
RSO_CLIENT_SECRET=*******************************
APP_BASE_URL=http://localhost.riotgames.com:3000
APP_CALLBACK_PATH=/oauth-callback
RESPONSE_TYPE=code
SCOPE=openid
```
<!-- # Sequence diagram 🟦➡️🟩⬅️
TBD

# Debug 🔎🐞
## Insomnia 🛏
TBD

## Postman 🚀
TBD -->

# FAQs 🤨❓
1. How do I get my RSO_CLIENT_ID?
    1. TBD
1. How do I get my RSO_CLIENT_SECRET?
    1. TBD
1. How do I get my DEPLOYMENT_API_KEY?
    1. TBD
1. How do I get my APP_BASE_URL?
    1. TBD
1. How do I get my APP_CALLBACK_PATH?
    1. TBD
1. Is RESPONSE_TYPE always the same?
    1. TBD
1. Is SCOPE always the same?
    1. TBD
